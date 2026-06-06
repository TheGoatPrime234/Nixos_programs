use clap::{ValueEnum};

use crate::utils::get::*;
use crate::utils::select::*;
use crate::installer::drives::drives_part;
use crate::usb::flash::*;
use crate::usb::core::*;

#[derive(ValueEnum, Clone, Debug)]
pub enum ListDebug {
    Drives,
    Taildevices,
    Select,
    Iso,
    Flash,
    Hardware,
}

pub fn list_debug(function: &ListDebug) {
    match function {
        ListDebug::Drives => {
            for i in get_drives(String::from("127.0.0.1")).blockdevices {
                println!(" - - - - - - -");
                println!("{}", i.name);
                println!("  {}", i.size);
                println!("  {}", i.device_type);
            };
            drives_part(&select_drive(&String::from("127.0.0.1"), false), true, &String::from("127.0.0.1"));
        },
        ListDebug::Taildevices => {
            println!("{:?}", get_taildevices());
        },
        ListDebug::Select => {
            println!("{:?}", get_taildevices());
            println!("{:?}", get_drives(String::from("127.0.0.1")));
        },
        ListDebug::Iso => {
            println!("{}", build_iso(&true));
        },
        ListDebug::Flash => {
            flash_usb(FlashMode::Local, String::from("127.0.0.1"), true);
        },
        ListDebug::Hardware => {
            let target_ip = select_host(get_taildevices());
            println!("{}", get_hardware(&target_ip));
        },
    }
}

