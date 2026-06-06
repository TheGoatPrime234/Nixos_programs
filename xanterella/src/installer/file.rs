use log::{debug, info, error};
use std::process::{self, Command};
use std::fs;

use crate::utils::get::*;

pub enum EditMode {
    Add,
    Remove,
}

pub fn create_hardware(config: String) {
    let file_path = format!("{}/hosts/crylia/hardware-configuration.nix", get_path(Paths::Nixconf));
    info!("[ RUN ] - Starte Erstellung der Hardware Config für Crylia");
    fs::write(&file_path, &config)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Hardware Config nicht schreiben: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Hardware Config für Crylia erstellt");
}

pub fn parse_config() -> String {
    let file_path = format!("{}/hosts/crylia/configuration.nix", get_path(Paths::Nixconf));
    info!("[ RUN ] - Starte pars für den Inhalt von Crylia");
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Config von Crylia nicht auslesen: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Parsed Content of Crylia");
    content
}

pub fn edit_config(content: String, mode: EditMode) -> String {
    let result = match mode {
        EditMode::Add => {
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
            whole_content
        },
        EditMode::Remove => {
            let whole_content = content.replace("    ./hardware-configuration.nix\n", "");
            debug!("Neuer Inhalt: \n{}", whole_content);
            whole_content
        },
    };
    result
}

pub fn write_config(content: String) {
    let file_path = format!("{}/hosts/crylia/configuration.nix", get_path(Paths::Nixconf));

    fs::write(file_path, content)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Config von Crylia nicht überschreiben: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Configuration von Crylia überschreiben");
}

pub fn remove_hardware() {
    let file_path = format!("{}/hosts/crylia/hardware-configuration.nix", get_path(Paths::Nixconf));
    fs::remove_file(file_path)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Hardware Config nicht löschen: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Hardware Config gelöscht");
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
