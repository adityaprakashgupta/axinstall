// Not sure of this code

use crate::args::DesktopSetup;
use crate::functions::partition::mount;
use crate::functions::*;
use crate::internal::exec::*;
use crate::internal::*;
use std::path::PathBuf;

pub fn install_bootloader_efi(efidir: PathBuf) {
    install::install(vec![
        "grub",
        "efibootmgr",
        "grub-btrfs",
    ]);
    let efidir = std::path::Path::new("/mnt").join(efidir);
    let efi_str = efidir.to_str().unwrap();
    if !std::path::Path::new(&format!("/mnt{efi_str}")).exists() {
        crash(format!("The efidir {efidir:?} doesn't exist"), 1);
    }
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=grub"),
                String::from("--removable"),
            ],
        ),
        "install AxOS grub as efi with --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=grub"),
            ],
        ),
        "install AxOS grub as efi without --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn remount(root: &str, oldroot: &str, efi: bool, efidir: &str, bootdev: &str, firstrun: bool) {
    if efi && firstrun {
        exec_eval(
            exec("umount", vec![String::from(bootdev)]),
            &format!("Unmount {}", bootdev),
        );
        exec_eval(
            exec("umount", vec![String::from(oldroot)]),
            "Unmount old root",
        );
        mount(root, "/mnt", "");
        exec_eval(
            exec("mkdir", vec![String::from("-p"), String::from(efidir)]),
            format!("Creating mountpoint {efidir} for {bootdev}").as_str(),
        );
        mount(bootdev, efidir, "");
    } else if efi && !firstrun {
        exec_eval(
            exec("umount", vec![String::from(bootdev)]),
            &format!("Unmount {}", bootdev),
        );
        exec_eval(
            exec("umount", vec![String::from(root)]),
            "Unmount root",
        );
        mount(oldroot, "/mnt", "");
        mount(bootdev, efidir, "");
    } else if !efi && firstrun {
        exec_eval(
            exec("umount", vec![String::from(bootdev)]),
            &format!("Unmount {}", bootdev),
        );
        exec_eval(
            exec("umount", vec![String::from(oldroot)]),
            "Unmount old root",
        );
        mount(root, "/mnt", "");
        exec_eval(
            exec("mkdir", vec![String::from("-p"), String::from("/mnt/boot")]),
            format!("Creating mountpoint /boot for {bootdev}").as_str(),
        );
        mount(bootdev, "/mnt/boot", "");
    } else if !efi && !firstrun {
        exec_eval(
            exec("umount", vec![String::from(bootdev)]),
            &format!("Unmount {}", bootdev),
        );
        exec_eval(
            exec("umount", vec![String::from(root)]),
            "Unmount root",
        );
        mount(oldroot, "/mnt", "");
        mount(bootdev, "/mnt/boot", "");
    } else {
        panic!("Unknown state");
    }
}

pub fn setup_arch(root: &str, oldroot: &str, efi: bool, efidir: &str, bootdev: &str) {
    log::debug!("Setting up AxOS");
    remount(root, oldroot, efi, efidir, bootdev, true);
    base::install_base_packages("linux".to_string());
    base::setup_archlinux_keyring();
    base::genfstab();
    locale::set_locale("en_US.UTF-8 UTF-8".to_string());
    locale::set_timezone("Europe/Berlin"); // TODO: get the proper timezone
    network::set_hostname("axos");
    network::create_hosts();
    exec_eval(
        exec(
            "sed",
            vec![
                String::from("-i"),
                String::from("-e"),
                String::from("s/AxOS/AxOS/g"),
                String::from("/mnt/etc/os-release"),
            ],
        ),
        "Change os-release",
    );
    if efi {
        install_bootloader_efi(PathBuf::from(efidir.replace("/mnt", "")));
    }
    users::root_pass("root"); // Change this to a secure password
    desktops::install_desktop_setup(DesktopSetup::Kde);
    users::new_user(
        "root",
        true,
        "root", // Change this to a secure password
        false,
        "/bin/bash",
    );
    install(vec!["gparted", "firefox"]);
    exec_eval(
        exec(
            "cp",
            vec![
                String::from("/tmp/installSettings.json"),
                String::from("/mnt/etc/installSettings.json"),
            ],
        ),
        "Copy installSettings.json to /etc/installSettings.json in Arch",
    );
    remount(root, oldroot, efi, efidir, bootdev, false);
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "Recreate grub.cfg in AxOS",
    );
}
