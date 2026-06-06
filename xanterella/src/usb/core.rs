use log::info;

use crate::usb::flash::*;
use crate::utils::select::*;

pub fn flash_usb(mode: FlashMode, ip: String, debug: bool) {
    info!("[ RUN ] - Start Flashing");
    flash_iso(select_drive(&ip, false), build_iso(&debug), &mode, &ip, &debug);
    info!("[ OK ] - Finished Flashing");
}
    
