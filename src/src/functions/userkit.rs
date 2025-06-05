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
    install(vec!["@developer"]);
}

fn install_hacks() {
    install(vec!["@hacker"]);
}

fn install_artist() {
    install(vec!["@artist"]);
}

fn install_office() {
    install(vec!["@office"]);
}

fn install_entertainment() {
    install(vec!["@entertainment"])
}
