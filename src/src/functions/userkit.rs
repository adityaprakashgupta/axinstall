use crate::args::UserKit;
use crate::internal::*;

pub fn install_userkit(kit: UserKit) {
    log::debug!("Installing {:?}", kit);

    match kit {
        UserKit::Developer => install_dev(),
        UserKit::Hacker => install_hacks(),
        UserKit::Artist => install_artist(),
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
    ]);
}


fn install_hacks() {
    install(vec![
        "nmap",
        "wireshark",
        "hydra",
        "recon-ng",
        "burpsuite", 
        "dirbuster", 
        "nikto",     
        "metasploit",
        "msfvenom",  
        "john",      
        "gdb",       
        "radare2",   
        "binwalk",   
        "curl",      
        "dnsutils",
        "netcat",
        "socat",
        "tcpdump",
        "nessus",
        "openvas",
        "lynis",
        "aircrack-ng",
        "ettercap",
        "macchanger",
        "dnschef",
        "tor",
        "proxychains",
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
        "pencil2d",
        "gimp-plugin-resynthesizer",
        "obs-studio",
        "krita-plugin-paintop",
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
