use std::process::{self, Command};
use log::{info, error, debug};
use std::fs;

use crate::generator::*;
use crate::get::*;

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
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte parted nicht starten: {}", err); 
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

    if !debug {
        let mkfs_efi = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mkfs.fat")
            .arg(drives_name(&primdrive, 1))
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
            .arg(drives_name(&primdrive, 2))
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

    info!("[ OK ] - Formatierungs & Partitionierungs Prozess erfolgreich");
}

pub fn drives_mount(primdrive: &String, ip: &String) {

        let root = Command::new("ssh")
            .arg(get_sshstring(&ip))
            .arg("mount")
            .arg(drives_name(&primdrive, 2))
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
            .arg(drives_name(&primdrive, 1))
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

    let system_path = fs::read_link(format!("{}/result", gen_path(Paths::Nixconf)))
        .unwrap_or_else(|err| { 
            error!("Konnte Symlink 'result' nicht auflösen: {}", err); 
            process::exit(1); 
        })
        .to_string_lossy()
        .into_owned();
    debug!("System-Pfad im Nix-Store: {}", system_path);
    info!("[ OK ] - Kopiere System-Closure auf Zielgerät");
    
    let copy = Command::new("nix")
        .env("NIX_SSHOPTS", "-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null")
        .args([
            "copy", 
            "--substitute-on-destination", 
            "--to", 
            &format!("ssh-ng://root@{}?remote-store=local%3Froot%3D/mnt", ip), 
            "./result"
        ])
        .current_dir(gen_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { 
            error!("Konnte nix copy nicht starten: {}", err); 
            process::exit(1); 
        });

    if !copy.status.success() {
        let err = String::from_utf8_lossy(&copy.stderr);
        error!("[ FAILED ] - Kopieren der System-Closure fehlgeschlagen:\n{}", err);
        process::exit(1);
    }

    info!("[ OK ] - Starte die Installation auf dem Zielgerät");
    info!("[ OK ] - Registriere System-Profil und installiere Bootloader");

    let profile_cmd = format!("nix-env --store /mnt -p /mnt/nix/var/nix/profiles/system --set {}", system_path);
    let profile = Command::new("ssh")
        .args(["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"])
        .arg(format!("root@{}", ip))
        .arg(&profile_cmd)
        .output()
        .unwrap_or_else(|err| { error!("SSH Fehler beim Profil setzen: {}", err); process::exit(1); });

    if !profile.status.success() {
        error!("[ FAILED ] - Profil-Registrierung fehlgeschlagen: {}", String::from_utf8_lossy(&profile.stderr));
        process::exit(1);
    }

    let activate_cmd = "nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/activate'";
    let activate = Command::new("ssh")
        .args(["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"])
        .arg(format!("root@{}", ip))
        .arg(activate_cmd)
        .output()
        .unwrap_or_else(|err| { error!("SSH Fehler bei der Aktivierung: {}", err); process::exit(1); });

    if !activate.status.success() {
        error!("[ FAILED ] - Systemaktivierung fehlgeschlagen: {}", String::from_utf8_lossy(&activate.stderr));
        process::exit(1);
    }

    let bootloader_cmd = "nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/bin/switch-to-configuration boot'";
    let bootloader = Command::new("ssh")
        .args(["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"])
        .arg(format!("root@{}", ip))
        .arg(bootloader_cmd)
        .output()
        .unwrap_or_else(|err| { error!("SSH Fehler beim Bootloader: {}", err); process::exit(1); });

    if !bootloader.status.success() {
        error!("[ FAILED ] - Bootloader Installation fehlgeschlagen: {}", String::from_utf8_lossy(&bootloader.stderr));
        process::exit(1);
    }

    info!("[ OK ] - Installation erfolgreich abgeschlossen! Das System kann neu gestartet werden.");
}
