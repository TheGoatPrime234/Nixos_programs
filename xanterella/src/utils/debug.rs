use clap::{ValueEnum};

use crate::installer::get::*;
use crate::installer::drives::drives_part;
use crate::utils::select::*;

#[derive(ValueEnum, Clone, Debug)]
pub enum ListDebug {
    Drives,
    Taildevices,
    Select,
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
        ListDebug::Hardware => {
            let target_ip = select_host(get_taildevices());
            println!("{}", get_hardware(&target_ip));
        },
    }
}

