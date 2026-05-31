use std::process::{self, Command};
use log::{info, error};

pub fn drives_part(primdrive: &String, debug: bool) {
    info!("[ OK ] - Starte formatierung und partitionierung");
    let drive = format!("/dev/{}", primdrive);
    let p_suffix = if primdrive.contains("nvme") || primdrive.contains("mmcblk") { 
        "p" 
    } else { 
        "" 
    };
    
    let efi_partition = format!("{}{}{}", drive, p_suffix, "1");
    let root_partition = format!("{}{}{}", drive, p_suffix, "2");

    if !debug {
            let parted_efi = Command::new("parted")
            .arg("-s")
            .arg(&drive)
            .args(["mklabel", "gpt"])
            .args(["mkpart", "ESP", "fat32", "1Mib", "512MiB"])
            .args(["set", "1", "esp", "on"])
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte parted nicht starten: {}", err); process::exit(1); });
        if !parted_efi.status.success() {
            let err = String::from_utf8_lossy(&parted_efi.stderr);
            error!("[ FAILED ] - Konnte das Drive nicht partitionieren: {}", err);
            process::exit(1);
        }
    };
    info!("[ OK ] - Efi Partition erstellt");

    if !debug {
        let parted_root = Command::new("parted")
            .arg("-s")
            .arg(&drive)
            .args(["mkpart", "primary", "ext4", "512MiB", "100%"])
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte parted nicht starten: {}", err); process::exit(1); });
        if !parted_root.status.success() {
            let err = String::from_utf8_lossy(&parted_root.stderr);
            error!("[ FAILED ] - Konnte das Drive nicht partitionieren: {}", err);
            process::exit(1);
        }
    };
    info!("[ OK ] - Root Partition erstellt");

    if !debug {
        let mkfs_efi = Command::new("mkfs.fat")
            .arg(&efi_partition)
            .args(["-F", "32"])
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Mkfs.ext4 nicht starten: {}", err); process::exit(1); });
        if !mkfs_efi.status.success() {
            let err = String::from_utf8_lossy(&mkfs_efi.stderr);
            error!("[ FAILED ] - Konnte die Partition nicht formatieren: {}", err);
            process::exit(1);
        }
    };
    info!("[ OK ] - Efi Partition formatiert");

    if !debug {
        let mkfs_root = Command::new("mkfs.ext4")
            .arg(&root_partition)
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Mkfs.ext4 nicht starten: {}", err); process::exit(1); });
        if !mkfs_root.status.success() {
            let err = String::from_utf8_lossy(&mkfs_root.stderr);
            error!("[ FAILED ] - Konnte die Partition nicht formatieren: {}", err);
            process::exit(1);
        }
    };
    info!("[ OK ] - Ext4 Partition formatiert");

    info!("[ OK ] - Drive fertig partitioniert");
}
