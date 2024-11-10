use crate::internal::exec::*;
use crate::internal::*;
use log::warn;
use std::path::PathBuf;

pub fn install_base_packages(kernel: String) {
    std::fs::create_dir_all("/mnt/etc").unwrap();
    let kernel_to_install = if kernel.is_empty() {
        "linux"
    } else {
        match kernel.as_str() {
            "linux" => "linux",
            "linux-lts" => "linux-lts",
            "linux-zen" => "linux-zen",
            "linux-hardened" => "linux-hardened",
            _ => {
                warn!("Unknown kernel: {}, using default instead", kernel);
                "linux"
            }
        }
    };
    install::install(vec![
        // Base Arch
        "base",
        "base-devel",
        kernel_to_install,
        format!("{kernel_to_install}-headers").as_str(),
        "linux-firmware",
        "networkmanager",
        "man-db",
        "man-pages",
        "nano",
        "sudo",
        "curl",
        "wget",
        "git",
        "archlinux-keyring",
        "grep",
        // Base AxOS
        "about-axos",
        "axos-hooks",
        "axos-hooks-extra",
        "plymouth-theme-axos",
        // Extra goodies
        "neofetch",
        "axos/plymouth",
        "plymouth-theme-axos",
        // Fonts
        "noto-fonts",
        "noto-fonts-emoji",
        "noto-fonts-cjk",
        "noto-fonts-extra",
        "ttf-nerd-fonts-symbols-common",
        // Common packages for all desktops
        "xterm",
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "power-profiles-daemon",
        "cups",
        "cups-pdf",
        "bluez",
        "bluez-cups",
        "bash-completion",
        "zsh-completions",
        "ttf-liberation",
        "dnsmasq",
        "networkmanager",
        "xdg-user-dirs",
        "firefox",
        "cmake",
        "python",
        "python-pip",
        "bash",
        "bash-completion",
        "deepin-icon-theme",
        "make",
        "inxi",
        "htop",
        "fwupd",
        "sddm-theme-chili",
        "power-profiles-daemon",
        "ntp",
        "kf6",
        // Graphic drivers
        "xf86-video-amdgpu",
        "xf86-video-intel",
        "xf86-video-nouveau",
        "xf86-video-vmware",
        "vulkan-intel",
        "vulkan-radeon",
        "vulkan-icd-loader",
        "virtualbox-guest-utils",
        // Chaotic-AUR
        "chaotic-keyring",
        "chaotic-mirrorlist",
    ]);
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");

    exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("bluetooth")],
        ),
        "Enable bluetooth",
    );

    exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("cups")],
        ),
        "Enable CUPS",
    );
}

pub fn setup_archlinux_keyring() {
    exec_eval(
        exec_chroot("pacman-key", vec![
            String::from("--init"),
        ]),
        "Initialize pacman keyring",
    );
    exec_eval(
        exec_chroot("pacman-key", vec![
            String::from("--populate"),
            String::from("archlinux"),
        ]),
        "Populate pacman keyring",
    );
}

pub fn genfstab() {
    exec_eval(
        exec(
            "bash",
            vec![
                String::from("-c"),
                String::from("genfstab -U /mnt >> /mnt/etc/fstab"),
            ],
        ),
        "Generate fstab",
    );
}

pub fn install_bootloader_efi(efidir: PathBuf) {
    install::install(vec![
        "axos/grub",
        "efibootmgr",
        "os-prober",
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
                String::from("--bootloader-id=axos"),
                String::from("--removable"),
            ],
        ),
        "install grub as efi with --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=axos"),
            ],
        ),
        "install grub as efi without --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn install_bootloader_legacy(device: PathBuf) {
    install::install(vec![
        "axos/grub",
        "os-prober",
    ]);
    if !device.exists() {
        crash(format!("The device {device:?} does not exist"), 1);
    }
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![String::from("--target=i386-pc"), device],
        ),
        "install grub as legacy",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn setup_timeshift() {
    install(vec!["timeshift", "timeshift-autosnap",]);
    exec_eval(
        exec_chroot("timeshift", vec![String::from("--create")]),
        "setup timeshift",
    )
}

pub fn copy_live_config() {
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");
    files::copy_file("/etc/axos-version", "/mnt/etc/axos-version");
    std::fs::create_dir_all("/mnt/etc/sddm.conf.d").unwrap();
    files::copy_file("/etc/sddm.conf.d/settings.conf", "/mnt/etc/sddm.conf.d/settings.conf");
    files::copy_file("/etc/mkinitcpio.conf", "/mnt/etc/mkinitcpio.conf");
}

pub fn install_flatpak() {
    install(vec!["flatpak"]);
    exec_eval(
        exec_chroot(
            "flatpak",
            vec![
                String::from("remote-add"),
                String::from("--if-not-exists"),
                String::from("flathub"),
                String::from("https://flathub.org/repo/flathub.flatpakrepo"),
            ],
        ),
        "add flathub remote",
    )
}

pub fn install_nvidia() {
    install(vec!["nvidia", "nvidia-utils", "nvidia-settings"]);
}

pub fn install_zram() {
    install(vec!["zram-generator"]);
    files::create_file("/mnt/etc/systemd/zram-generator.conf");
    files_eval(
        files::append_file("/mnt/etc/systemd/zram-generator.conf", "[zram0]"),
        "Write zram-generator config",
    );
}
