use crate::generator::*;

use std::process::{self, Command};
use log::{debug, info, error};
use walkdir:: WalkDir;
use std::io;

pub fn edit_pars_files() -> Vec<String> {
    let files: Vec<String> = WalkDir::new(&gen_path(Paths::Nixconf)
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
    debug!("{}", files);
    files
}

pub fn edit_add_host(name: String, _ip: String) {
    let file_path = format!("{}/hosts/{}/configuration.nix", gen_path(Paths::Nixconf), name);
    fs::write(&file_path, &content)
        .unwrap_or_else(|err| { error!("[ FAILED ] - Konnte die Configdatei nicht schreiben: {}", err); process::exit(1); });
}
