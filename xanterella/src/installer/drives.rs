use std::process::{self, Command};
use log::{info, error};

use crate::utils::get::*;

pub fn drives_part(primdrive: &String, debug: bool, ip: &String) {
    info!("[ OK ] - Starte formatierung und partitionierung");
    let drive = format!("/dev/{}", primdrive);
    
    if !debug {
        let parted_efi = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("parted")
            .arg("-s")
            .arg(&drive)
            .args(["mklabel", "gpt"])
            .args(["mkpart", "ESP", "fat32", "1Mib", "512MiB"])
            .args(["set", "1", "esp", "on"])
            .output()
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte 'parted' nicht starten: {}", err); 
                process::exit(1); 
            });
        if !parted_efi.status.success() {
            error!("[ FAILED ] - Konnte das Drive nicht partitionieren: {}", String::from_utf8_lossy(&parted_efi.stderr));
            process::exit(1);
        }
    };
    info!("[ OK ] - Efi Partition erstellt");

    if !debug {
        let parted_root = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("parted")
            .arg("-s")
            .arg(&drive)
            .args(["mkpart", "primary", "ext4", "512MiB", "100%"])
            .output()
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte parted nicht starten: {}", err); 
                process::exit(1); 
            });
        if !parted_root.status.success() {
            error!("[ FAILED ] - Konnte das Drive nicht partitionieren: {}", String::from_utf8_lossy(&parted_root.stderr));
            process::exit(1);
        }
    };
    info!("[ OK ] - Root Partition erstellt");

    info!("[ OK ] - Partitionierungs Prozess erfolgreich");
}

pub fn drives_format(primdrive: &String, debug: bool, ip: &String) {
    if !debug {
        let mkfs_efi = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mkfs.fat")
            .arg(get_drives_name(&primdrive, 1))
            .args(["-F", "32"])
            .output()
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte Mkfs.ext4 nicht starten: {}", err); 
                process::exit(1); 
            });
        if !mkfs_efi.status.success() {
            error!("[ FAILED ] - Konnte die Partition nicht formatieren: {}", String::from_utf8_lossy(&mkfs_efi.stderr));
            process::exit(1);
        }
    };
    info!("[ OK ] - Efi Partition formatiert");

    if !debug {
        let mkfs_root = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mkfs.ext4")
            .arg(get_drives_name(&primdrive, 2))
            .output()
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte Mkfs.ext4 nicht starten: {}", err); 
                process::exit(1); 
            });
        if !mkfs_root.status.success() {
            error!("[ FAILED ] - Konnte die Partition nicht formatieren: {}", String::from_utf8_lossy(&mkfs_root.stderr));
            process::exit(1);
        }
    };
    info!("[ OK ] - Ext4 Partition formatiert");

    info!("[ OK ] - Formatierungs Prozess erfolgreich");
}

pub fn drives_mount(primdrive: &String, ip: &String) {

        let root = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mount")
            .arg(get_drives_name(&primdrive, 2))
            .arg("/mnt")
            .output()
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte mount nicht starten: {}", err); 
                process::exit(1); 
            });
        if !root.status.success() {
            error!("[ FAILED ] - Konnte die Root Partition nicht mounten: {}", String::from_utf8_lossy(&root.stderr));
            process::exit(1);
        }
        info!("[ OK ] - Root Partition gemounted");

        let dir = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mkdir")
            .arg("-p")
            .arg("/mnt/boot")
            .output()
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte mkdir nicht starten: {}", err); 
                process::exit(1); 
            });
        if !dir.status.success() {
            error!("[ FAILED ] - Konnte die den Boot Ordner nicht erstellen: {}", String::from_utf8_lossy(&dir.stderr));
            process::exit(1);
        }

        let boot = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mount")
            .arg(get_drives_name(&primdrive, 1))
            .arg("/mnt/boot")
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte mount nicht starten: {}", err); process::exit(1); });
        if !boot.status.success() {
            error!("[ FAILED ] - Konnte die Boot Partition nicht mounten: {}", String::from_utf8_lossy(&boot.stderr));
            process::exit(1);
        }
        info!("[ OK ] - Boot Partition gemounted");

        info!("[ OK ] - Mount Prozess erfolgreich");
}
