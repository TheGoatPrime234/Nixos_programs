use log::info;

use crate::utils::check::*;

pub fn ping_full(ip: &String) {
    info!("[ RUN ] - Starte Ping Tests");

    ping(ip);
    ping_ssh(ip);
    info!("[ OK ] - Ping Tests erfolgreich");
}
