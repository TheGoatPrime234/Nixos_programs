use log::{debug, info, error};
use std::process::{self, Command};
use std::fs;

use crate::installer::get::*;

pub fn files_crylia_start(config: String) {
    let file_path1 = format!("{}/hosts/crylia/configuration.nix", get_path(Paths::Nixconf));
    let file_path2 = format!("{}/hosts/crylia/hardware-configuration.nix", get_path(Paths::Nixconf));

    fs::write(&file_path2, &config)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Hardware Config nicht schreiben: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Hardware Config für Crylia erstellt");

    let content = fs::read_to_string(&file_path1)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Config von Crylia nicht auslesen: {}", err); 
            process::exit(1); 
        });
    let Some((anfang, ende)) = content.split_once("  imports = [") else {
        error!("[ FAILED ] - Konnte 'imports = [' nicht in der Config von Crylia finden");
        process::exit(1);
    };
    let whole_content = format!(
        "{}
        imports = [
        ./hardware-configuration.nix
        {}", anfang, ende);

    debug!("Neuer Inhalt: \n{}", whole_content);

    fs::write(&file_path1, &whole_content)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Config von Crylia nicht überschreiben: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Configuration von Crylia überschreiben");

    files_alejandra();
}

pub fn files_crylia_finish() {
    let file_path1 = format!("{}/hosts/crylia/configuration.nix", get_path(Paths::Nixconf));
    let file_path2 = format!("{}/hosts/crylia/hardware-configuration.nix", get_path(Paths::Nixconf));

    let content = fs::read_to_string(&file_path1)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Config von Crylia nicht auslesen: {}", err); 
            process::exit(1); 
        });
    let whole_content = content.replace("    ./hardware-configuration.nix\n", "");
    debug!("Neuer Inhalt: \n{}", whole_content);

    fs::write(&file_path1, &whole_content)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Config von Crylia nicht überschreiben: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Configuration von Crylia überschreiben");

    fs::remove_file(file_path2)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Hardware Config nicht löschen: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Hardware Config gelöscht");

    files_alejandra();
}

pub fn files_alejandra() {
    let alejandra = Command::new("alejandra")
        .arg(".")
        .current_dir(get_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte Alejandra nicht starten: {}", err); 
            process::exit(1); 
        });
    debug!("Alejandra: \n{}", String::from_utf8_lossy(&alejandra.stdout));
    if !alejandra.status.success() {
        error!("[ FAILED ] - Konnte die Dateien mit Alejandra nicht formatieren: {}", String::from_utf8_lossy(&alejandra.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Dateien wurden mit Alejandra formatiert");
}

/*
pub fn edit_pars_files() -> Vec<String> {
    let files: Vec<String> = WalkDir::new(&get_path(Paths::Nixconf))
        .sort_by_file_name()
        .contents_first(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect();
    info!("[ OK ] - Nixos Config Dateien geparst");
    debug!("{:#?}", files);
    files
}

pub fn edit_add_host(name: String, _ip: String) {
    let file_path = format!("{}/hosts/{}/configuration.nix", get_path(Paths::Nixconf), name);
    fs::write(&file_path, &content)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Configdatei nicht schreiben: {}", err); 
            process::exit(1); 
        });
}
*/
