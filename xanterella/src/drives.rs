use std::process::{self, Command};
use log::{info, error};

use crate::*;

pub fn drives_name(primdrive: &String, number: i8) -> String {
    let drive = format!("/dev/{}", primdrive);
    let p_suffix = if primdrive.contains("nvme") || primdrive.contains("mmclblk") {
        "p"
    } else {
        ""
    };
    let partition = format!("{}{}{}", drive, p_suffix, number);
    partition
}


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
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte parted nicht starten: {}", err); process::exit(1); });
        if !parted_efi.status.success() {
            let err = String::from_utf8_lossy(&parted_efi.stderr);
            error!("[ FAILED ] - Konnte das Drive nicht partitionieren: {}", err);
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
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte parted nicht starten: {}", err); process::exit(1); });
        if !parted_root.status.success() {
            let err = String::from_utf8_lossy(&parted_root.stderr);
            error!("[ FAILED ] - Konnte das Drive nicht partitionieren: {}", err);
            process::exit(1);
        }
    };
    info!("[ OK ] - Root Partition erstellt");

    if !debug {
        let mkfs_efi = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mkfs.fat")
            .arg(drives_name(&primdrive, 1))
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
        let mkfs_root = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mkfs.ext4")
            .arg(drives_name(&primdrive, 2))
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte Mkfs.ext4 nicht starten: {}", err); process::exit(1); });
        if !mkfs_root.status.success() {
            let err = String::from_utf8_lossy(&mkfs_root.stderr);
            error!("[ FAILED ] - Konnte die Partition nicht formatieren: {}", err);
            process::exit(1);
        }
    };
    info!("[ OK ] - Ext4 Partition formatiert");

    info!("[ OK ] - Formatierungs & Partitionierungs Prozess erfolgreich");
}

pub fn drives_mount(primdrive: String, ip: String) {

        let boot = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mount")
            .arg(drives_name(&primdrive, 1))
            .arg("/mnt/boot")
            .arg("-p")
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte mount nicht starten: {}", err); process::exit(1); });
        if !boot.status.success() {
            let err = String::from_utf8_lossy(&boot.stderr);
            error!("[ FAILED ] - Konnte die Boot Partition nicht mounten: {}", err);
            process::exit(1);
        }
        info!("[ OK ] - Boot Partition gemounted");

        let root = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mount")
            .arg(drives_name(&primdrive, 1))
            .arg("/mnt")
            .output()
            .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte mount nicht starten: {}", err); process::exit(1); });
        if !root.status.success() {
            let err = String::from_utf8_lossy(&root.stderr);
            error!("[ FAILED ] - Konnte die Root Partition nicht mounten: {}", err);
            process::exit(1);
        }
        info!("[ OK ] - Root Partition gemounted");

        info!("[ OK ] - Mount Prozess erfolgreich");
}

pub fn build_and_deploy(ip: &String) {
    info!("[ OK ] - Starte Build and Deployment");
    
    let build = Command::new("nix")
        .args(["build", ".#nixosConfigurations.crylia.config.system.build.toplevel"])
        .current_dir(gen_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { 
            error!("Konnte nix build nicht starten: {}", err); 
            process::exit(1); 
        });
    if !build.status.success() {
        error!("[ FAILED ] - Lokaler Build fehlgeschlagen: {}", String::from_utf8_lossy(&build.stderr));
        process::exit(1);
    }

    let result_symlink = format!("{}/result", gen_path(Paths::Nixconf);

    let system_path = fs::read_link(&result_symlink)
        .unwrap_or_else(|err| { 
            error!("Konnte Symlink 'result' nicht auflösen: {}", err); 
            process::exit(1); 
        })
        .to_string_lossy()
        .into_owned();
    
    debug!("System-Pfad im Nix-Store lautet: {}", system_path);

    info!("[ OK ] - Kopiere System-Closure auf Zielgerät");
    env::set_var("NIX_SSHOPTS", "-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null");
    
    let copy = Command::new("nix")
        .args(["copy", "--to", &format!("ssh://root@{}", ip), "./result"])
        .current_dir(&flake_dir)
        .output()
        .unwrap_or_else(|err| { error!("Konnte nix copy nicht starten: {}", err); process::exit(1); });
    if !copy.status.success() {
        error!("[ FAILED ] - Kopieren fehlgeschlagen: {}", String::from_utf8_lossy(&copy.stderr));
        process::exit(1);
    }

    info!("[ OK ] - Starte die Installation auf dem Zielgerät...");
    let install_cmd = format!("nixos-install --system {} --root /mnt --no-root-passwd", system_path);
    
    let install = Command::new("ssh")
        .args(["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"])
        .arg(format!("root@{}", ip))
        .arg(&install_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte SSH nicht starten: {}", err); process::exit(1); });
    if !install.status.success() {
        error!("[ FAILED ] - nixos-install fehlgeschlagen: {}", String::from_utf8_lossy(&install.stderr));
        process::exit(1);
    }

    info!("[ OK ] - Installation erfolgreich abgeschlossen!");
}
