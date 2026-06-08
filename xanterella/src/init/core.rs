use log::info;

use crate::init::init::*;

pub fn init_git(ip: &str) {
    info!("[ RUN ] - Starte Git und GitHub Authentikation");

    init_git_email(ip);
    init_git_name(ip);
    init_github(ip);
    info!("[ OK ] - Git und GitHub Authentikation erfolgreich");
}
