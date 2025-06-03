
use crate::args::DesktopSetup;
use crate::internal::exec::*;
use crate::internal::*;

pub fn install_desktop_setup(desktop_setup: DesktopSetup) {
    log::debug!("Installing {:?}", desktop_setup);
    match desktop_setup {
        DesktopSetup::Kde => install_kde(),
        DesktopSetup::Calla => install_calla(),
        DesktopSetup::Sleex => install_sleex(),
        DesktopSetup::None => log::debug!("No desktop setup selected"),
    }
    install_networkmanager();
}

fn install_networkmanager() {
    install(vec!["networkmanager"]);
    exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("NetworkManager")],
        ),
        "Enable network manager",
    );
}

fn install_calla() {
    install(vec![
        "calla",
        "alacritty",
        "nautilus",
        "polkit-gnome",
        "cbatticon",
        "blueman",
        "ttf-roboto",
        "noto-fonts-emoji",
        "ttf-material-icons-git",
        "ttf-material-design-icons-extended",
        "playerctl",
        "redshift",
        "xsettingsd",
        "galculator",
        "baobab",
        "gnome-characters",
        "mousepad",
        "gparted",
        "wmctrl",
        "libinput-gestures",
        "lollypop",

    ]);
    enable_dm("sddm");
}


fn install_kde() {
    install(vec![
        "xorg",
        "plasma-desktop",
        "kde-utilities",
        "kde-system",
        "axskel",
        "papirus-icon-theme",
        "sddm",
        "okular",
        "kate",
        "dolphin",
        "konsole",
        "ark",
        "kdeconnect",
        "plasma-systemmonitor",
        "discover",
        "filelight",
        "kcalc",
        "partitionmanager",
        "kwrite",
        "plasma-pa",
        "kscreen",
        "kdialog",
        "print-manager",
        "kde-gtk-config",
        "kinfocenter",
        "libreoffice-fresh",
        "sddm-theme-chili",
        "bluez-qt",
        "plasma-nm",
        "spectacle",
        "plasma-kcm",
    ]);
    enable_dm("sddm");
}


fn install_sleex() {
    install(vec![
        // Hyprland stuff
        "hyprland",
        "hyprlang",
        "hyprcursor",
        "hyprutils",
        "hyprlock",
        "hyprpicker",
        "hyprwayland-scanner",
        // AxOS stuff
        "sleex",
        "sleex-optional",
        ]);
    enable_dm("sddm");
    set_sddm_sleex_default();
}

fn set_sddm_sleex_default() {
    exec_eval(
        exec_chroot(
            "mv",
            vec![
                String::from("/usr/share/wayland-sessions/hyprland.desktop"),
                String::from("/usr/share/wayland-sessions/hyprland.desktop.hidden"),
            ],
        ),
        "Rename hyprland.desktop to hyprland.desktop.hidden",
    );
    exec_eval(
        exec_chroot(
            "mv",
            vec![
                String::from("/usr/share/wayland-sessions/hyprland-uwsm.desktop"),
                String::from("/usr/share/wayland-sessions/hyprland-uwsm.desktop.hidden"),
            ],
        ),
        "Rename hyprland-uwsm.desktop to hyprland-uwsm.desktop.hidden",
    );
}

fn enable_dm(dm: &str) {
    log::debug!("Enabling {}", dm);
    exec_eval(
        exec_chroot("systemctl", vec![String::from("enable"), String::from(dm)]),
        format!("Enable {}", dm).as_str(),
    );
}