use crate::installer::install::*;
use crate::installer::file::*;
use crate::installer::drives::*;
use crate::utils::git::git_full;
use crate::utils::check::*;
use crate::utils::select::*;
use crate::utils::get::*;

pub fn remote_install(automate: &bool, fast: &bool) {
    let target_ip = select_host(get_taildevices());
    ssh_ping(&target_ip);
    crylia_edit_start(get_hardware(&target_ip));
    git_full(String::from("Xanterella Remote-Install"));
    if !*fast {
        nix_check();
    };
    let primdrive = select_drive(&target_ip, *automate);
    drives_part(&primdrive, true, &target_ip);
    //drives_format(&primdrive, true, &target_ip);
    drives_mount(&primdrive, &target_ip);
    build();
    deploy(&target_ip);
    reboot(&target_ip);
    // -----------------------------------------------------
    crylia_edit_finish();
    git_full(String::from("Xanterella Remote-Install cleanup"));
}

pub fn clean() {
    crylia_edit_finish();
    git_full(String::from("Xanterella Remote-Install cleanup"));
}

pub fn crylia_edit_start(config: String) {
    create_hardware(config);
    write_config(edit_config(parse_config(), EditMode::Add));
    files_alejandra();
}

pub fn crylia_edit_finish() {
    remove_hardware(); 
    write_config(edit_config(parse_config(), EditMode::Remove));
    files_alejandra();
}

pub fn drives_part(primdrive: &String, debug: bool, ip: &String) {
    let drive = format!("/dev/{}", primdrive);
    part_efi(&drive, debug, ip);
    part_root(&drive, debug, ip);
}
