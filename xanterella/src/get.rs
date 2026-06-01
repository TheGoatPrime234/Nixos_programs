use crate::generator::*;

use std::process::{self, Command};
use log::{debug, info, error};
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Drives {
    pub blockdevices: Vec<BlockDevice>,
}

#[derive(Deserialize, Debug)]
pub struct BlockDevice {
    pub name: String,
    pub size: String,

    #[serde(rename = "type")]
    pub device_type: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Taildevices {
    #[serde(rename = "Peer")]
    pub devices: HashMap<String, DeviceInfo>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DeviceInfo {
    #[serde(rename = "HostName")]
    pub name: String,
    #[serde(rename = "TailscaleIPs")]
    pub ip: Vec<String>,
    #[serde(rename = "OS")]
    pub os: String,
}

pub fn get_ssh_hardware(ip: &String) -> String {
    let ssh_command = format!("root@{}", ip);
    let ssh = Command::new("ssh")
        .arg(&ssh_command)
        .arg("nixos-generate-config --no-filesystems --show-hardware-config")
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte SSH nicht starten: {}", err); process::exit(1); });
    if !ssh.status.success() {
        let err = String::from_utf8_lossy(&ssh.stderr);
        error!("[ FAILED ] - Fehler beim erstellen der Hardware Config: {}", err);
        process::exit(1);
    }

    let hardware_config = String::from_utf8_lossy(&ssh.stdout).to_string();
    info!("[ OK ] - Hardware Config erstellt");
    debug!("{}", hardware_config);
    hardware_config
}

pub fn get_drives(ip: String) -> Drives {
    let parsed_drives;

    let ssh_command_root = format!("root@{}", ip);
    let lsblk = Command::new("ssh")
        .arg(&ssh_command_root)
        .arg("lsblk")
        .arg("--json")
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte lsblk nicht starten: {}", err); process::exit(1); });
    if !lsblk.status.success() {
        let err = String::from_utf8_lossy(&lsblk.stderr);
        error!("[ FAILED ] - Fehler beim Auslesen der als root Partitionen: {}", err);

        let ssh_command_cato = format!("cato@{}", ip);
        let lsblk1 = Command::new("ssh")
            .arg(&ssh_command_cato)
            .arg("lsblk")
            .arg("--json")
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte lsblk nicht starten: {}", err); process::exit(1); });
        if !lsblk1.status.success() {
            let err = String::from_utf8_lossy(&lsblk1.stderr);
            error!("[ FAILED ] - Fehler beim Auslesen der als cato Partitionen: {}", err);
            process::exit(1);

        } else {
            info!("[ OK ] - Drives mit Cato geparsen");
            parsed_drives = serde_json::from_slice::<Drives>(&lsblk1.stdout)
                .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte lsblk nicht parsen: {}", err); process::exit(1); });
        }
    } else {
        info!("[ OK ] - Drives mit Root geparsen");
        parsed_drives = serde_json::from_slice::<Drives>(&lsblk.stdout)
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte lsblk nicht parsen: {}", err); process::exit(1); });
    }
    info!("[ OK ] - Drives erfasst");
    info!("[ OK ] - Drives geparset");
    parsed_drives
}

pub fn get_taildevices() -> Taildevices {

    let tail_status = Command::new("tailscale")
        .arg("status")
        .arg("--json")
        .output()
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte 'tailscale status --json' nicht ausführen: {}", err); process::exit(1); });
    if !tail_status.status.success() {
        let err = String::from_utf8_lossy(&tail_status.stderr);
        error!("[ FAILED ] - Tailscale Status ist Fehlgeschlagen, bist du eingelogt, wurde das JSON nicht richtig geparst: {}", err);
        process::exit(1);
    }

    info!("[ OK ] - Fetched Tailscale Devices");
    serde_json::from_slice::<Taildevices>(&tail_status.stdout)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte den Output von Tailscale nicht parsen: {}", err); process::exit(1); })
}
