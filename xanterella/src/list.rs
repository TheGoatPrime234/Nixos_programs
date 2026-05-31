use clap::{ValueEnum};
use log::{debug, info, error};
use std::process::{self, Command};

use crate::*;

#[derive(ValueEnum, Clone, Debug)]
pub enum ListDebug {
    Drives,
    Taildevices,
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
            select_drive(&String::from("127.0.0.1"), false);
        },
        ListDebug::Taildevices => {
            println!("{:?}", get_taildevices());
        },
        ListDebug::Hardware => {
            let target_ip = select_host(get_taildevices());
            println!("{}", get_ssh_hardware(&target_ip));
        },
    }
}

