use std::vec;

use crate::args::UserKit;
use crate::internal::*;

pub fn install_userkit(kit: UserKit) {
    log::debug!("Installing {:?}", kit);

    match kit {
        UserKit::Developer => install_dev(),
        UserKit::Hacker => install_hacks(),
        UserKit::Artist => install_artist(),
        UserKit::Entertainment => install_entertainment(),
        UserKit::Office => install_office(),
    }
}

fn install_dev() {
    install(vec![
        "git",
        "vim",
        "neovim",
        "base-devel", 
        "cmake",
        "nodejs",
        "npm",
        "python",
        "python-pip",
        "python-virtualenv",
        "python-pipx",
        "rustup",
        "docker",
        "htop",
        "curl",
        "ripgrep",
        "exa",
        "tmux",
        "opencl-mesa",
        "rust",
    ]);
}


fn install_hacks() {
    install(vec![
        "nmap",
        "wireshark-qt",
        "hydra",
        "nikto",
        "john",
        "gdb",
        "radare2",
        "binwalk",
        "curl",
        "dnsutils",
        "netcat",
        "socat",
        "tcpdump",
        "htop",
        "tmux",
    ]);
}



fn install_artist() {
    install(vec![
        "gimp",
        "krita",
        "inkscape",
        "blender",
        "obs-studio",
        "fontforge",
        "darktable",
        "shotcut",
        "xournalpp",
        "xclip",
        "lmms",
        "audacity",
        "ffmpeg",
        "imagemagick",
    ]);
}


fn install_office() {
    install(vec![
        "libreoffice-fresh",      // Office suite
        "evince",                 // PDF/document viewer (GTK)
        "hunspell",               // Spell checker
        "pandoc",                 // Document converter
        "abiword",                // Lightweight word processor
        "gnumeric",               // Lightweight spreadsheet
        "calibre",                // E-book management
        "zathura",                // Lightweight PDF viewer
        "simple-scan",            // Scanning utility
        "xournalpp",              // Note-taking and PDF annotation
        "masterpdfeditor-free",   // PDF editor (if available in your repos)
    ]);
}

fn install_entertainment() {
    install(vec![
        "axuralis",
        "vlc",
        "mpv",
        "vesktop",
        "lutris"
    ])
}