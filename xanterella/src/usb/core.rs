use log::info;

use crate::usb::flash::*;
use crate::utils::select::*;
use crate::utils::get::*;

pub fn flash_usb(mode: FlashMode, ip: String, debug: bool) {
    let mode: FlashMode = select_mode("In welchem Modus soll geflasht werden: ");
    info!("[ RUN ] - Starte Flashing");

    match mode {
        FlashMode::Local => {
            let ip = String::from("127.0.0.1");
            flash_iso(select_drive(&ip, false), build_iso(&debug), &mode, &ip, &debug);
        },
        FlashMode::Remote => {
            let ip = select_host(get_taildevices());
            flash_iso(select_drive(&ip, false), build_iso(&debug), &mode, &ip, &debug);
        },
    }
    info!("[ OK ] - Flashing erfolgreich");
}
    
