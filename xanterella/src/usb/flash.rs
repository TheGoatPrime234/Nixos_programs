use log::{info, error};
use strum_macros::{Display, EnumIter};

use std::process::{self, Command};

use crate::utils::get::*;

#[derive(Debug, Clone, EnumIter, Display)]
pub enum FlashMode {
    Local,
    Remote,
}

pub fn build_iso(debug: &bool) -> String {
    info!("[ RUN ] - Starte ISO Build");

    if !debug {
        let build = Command::new("nix")
            .arg("build")
            .arg(".#nixosConfigurations.installer.config.system.build.isoImage")
            .arg("--impure")
            .current_dir(get_path(Paths::Nixconf))
            .output()
            .unwrap_or_else(|err| { 
                error!("Konnte nix build nicht starten: {}", err); 
                process::exit(1); 
            });
        if !build.status.success() {
            error!("[ FAILED ] - Lokaler ISO Build fehlgeschlagen: {}", String::from_utf8_lossy(&build.stderr));
            process::exit(1);
        }
    }
    info!("[ OK ] - ISO Build erfolgreich");
    get_iso()
}

pub fn flash_iso(drive: String, iso_path: String, _mode: &FlashMode, _ip: &String, debug: &bool) {
    info!("[ RUN ] - Starte USB flash");

    if !debug {
        let dd = Command::new("sudo")
            .arg("dd")
            .arg("bs=4M")
            .arg("conv=fsync")
            .arg("oflag=direct")
            .arg(format!("if={}", iso_path))
            .arg(format!("of=/dev/{}", drive))
            .output()
            .unwrap_or_else(|err| { 
                error!("Konnte 'dd' nicht starten: {}", err); 
                process::exit(1); 
            });
        if !dd.status.success() {
            error!("[ FAILED ] - Konnte den USB nicht flashen: {}", String::from_utf8_lossy(&dd.stderr));
            process::exit(1);
        }
    }
    info!("[ OK ] - USB flash erfolgreich");
}
