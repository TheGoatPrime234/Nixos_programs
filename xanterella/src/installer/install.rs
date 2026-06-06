use log::{info, error, debug};

use std::process::{self, Command};
use std::fs;

use crate::utils::get::*;

pub fn build() {
    info!("[ RUN ] - Starte loken Build");
    
    let build = Command::new("nix")
        .args(["build", ".#nixosConfigurations.crylia.config.system.build.toplevel"])
        .current_dir(get_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { 
            error!("Konnte nix build nicht starten: {}", err); 
            process::exit(1); 
        });
    if !build.status.success() {
        error!("[ FAILED ] - Lokaler Build fehlgeschlagen: {}", String::from_utf8_lossy(&build.stderr));
        process::exit(1);
    }

    info!("[ OK ] - Build erfolgreich");
}

pub fn copy(ip: &String) {
    info!("[ RUN ] - Starte Copy des Closure");
    let copy = Command::new("nix")
        .env("NIX_SSHOPTS", "-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null")
        .args([
            "copy", 
            "--no-check-sigs",
            "--substitute-on-destination", 
            "--to", 
            &format!("ssh-ng://root@{}?remote-store=local%3Froot%3D/mnt", ip), 
            "./result"
        ])
        .current_dir(get_path(Paths::Nixconf))
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
    info!("[ OK ] - Closure Copy erfolgreich");
}

pub fn profile(ip: &String) {
    let system_path = fs::read_link(format!("{}/result", get_path(Paths::Nixconf)))
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte Symlink 'result' nicht auflösen: {}", err); 
            process::exit(1); 
        })
        .to_string_lossy()
        .into_owned();
    debug!("System-Pfad im Nix-Store: {}", system_path);
    let profile_cmd = format!("nix-env --store /mnt -p /mnt/nix/var/nix/profiles/system --set {}", system_path);
    info!("[ RUN ] - Aktivierung des Profiles");
    let profile = Command::new("ssh")
        .arg(get_sshstring(ip))
        .args(["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"])
        .arg(&profile_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte 'ssh' oder 'nix' nicht starten: {}", err); process::exit(1); });
    if !profile.status.success() {
        error!("[ FAILED ] - Profil-Registrierung fehlgeschlagen: {}", String::from_utf8_lossy(&profile.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Aktivierung des Profiles erfolgreich");
}

pub fn prep(ip: &String) {
    let prep_cmd = "mkdir -m 0755 -p /mnt/etc && touch /mnt/etc/NIXOS";

    info!("[ RUN ] - Bereite Dateisystem für nixos-enter vor");
    let prep = Command::new("ssh")
        .args(["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"])
        .arg(get_sshstring(ip))
        .arg(prep_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte 'ssh' oder 'nix' nicht starten: {}", err); process::exit(1); });

    if !prep.status.success() {
        error!("[ FAILED ] - Vorbereitung fehlgeschlagen: {}", String::from_utf8_lossy(&prep.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Vorbereitung erfolgreich");
}

pub fn activate(ip: &String) {
    let activate_cmd = "NIXOS_INSTALL_BOOTLOADER=1 nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/activate'";
    info!("[ RUN ] - Aktiviere das System");
    let activate = Command::new("ssh")
        .arg(get_sshstring(ip))
        .args(["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"])
        .arg(activate_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte 'ssh' oder 'nix' nicht starten: {}", err); process::exit(1); });
    if !activate.status.success() {
        error!("[ FAILED ] - Systemaktivierung fehlgeschlagen: {}", String::from_utf8_lossy(&activate.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Aktivierung des Systems erfolgreich");
}

pub fn bootloader(ip: &String) {
    let bootloader_cmd = "nixos-enter --root /mnt --command 'NIXOS_INSTALL_BOOTLOADER=1 /nix/var/nix/profiles/system/bin/switch-to-configuration boot'";
    info!("[ RUN ] - Aktualisiere Bootloader");
    let bootloader = Command::new("ssh")
        .arg(get_sshstring(ip))
        .args(["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"])
        .arg(bootloader_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte 'ssh' oder 'nix' nicht starten: {}", err); process::exit(1); });
    if !bootloader.status.success() {
        error!("[ FAILED ] - Bootloader Installation fehlgeschlagen: {}", String::from_utf8_lossy(&bootloader.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Aktualisierung des Bootloaders erfolgreich");
}

pub fn reboot(ip: &String) {
    info!("[ RUN ] - System wird neugestartet");

    let _reboot = Command::new("ssh")
        .arg(get_sshstring(ip))
        .arg("reboot")
        .spawn()
        .unwrap_or_else(|err| { 
            error!("Konnte 'ssh' oder 'reboot' nicht starten: {}", err); 
            process::exit(1); 
        });

    info!("System neugestartet");
}
