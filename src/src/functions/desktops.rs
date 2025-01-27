
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
        "firefox",
        "galculator",
        "baobab",
        "gnome-characters",
        "mousepad",
        "gparted",
        "xterm",
        "wmctrl",
        "libinput-gestures",
        "wireplumber",
        "bash-completion",
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
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "papirus-icon-theme",
        "wireplumber",
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
        "networkmanager",
        "kscreen",
        "kdialog",
        "print-manager",
        "kde-gtk-config",
        "xdg-user-dirs",
        "kinfocenter",
        "libreoffice-fresh",
        "sddm-theme-chili",
        "packagekit-qt5",
        "power-profiles-daemon",
        "bluez",
        "bluez-qt",
    ]);
    enable_dm("sddm");
}


fn install_sleex() {
    install(vec![
        "sleex",
        "illogical-impulse-optional",
        ]);
    enable_dm("sddm");
    // TODO: set Sleex as the default session by removing the other's entries
}

fn enable_dm(dm: &str) {
    log::debug!("Enabling {}", dm);
    exec_eval(
        exec_chroot("systemctl", vec![String::from("enable"), String::from(dm)]),
        format!("Enable {}", dm).as_str(),
    );
}