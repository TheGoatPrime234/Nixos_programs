use crate::installer::get::*;
use crate::installer::install::*;
use crate::installer::file::*;
use crate::installer::drives::*;
use crate::utils::git::git_full;
use crate::utils::check::*;
use crate::utils::select::*;

pub fn remote_install(automate: &bool, fast: &bool) {
    let target_ip = select_host(get_taildevices());
    ssh_ping(&target_ip);
    get_hardware(&target_ip);
    files_crylia_start(get_hardware(&target_ip));
    git_full(String::from("Xanterella Remote-Install"));
    if !*fast {
        nix_check();
    };
    let primdrive = select_drive(&target_ip, *automate);
    drives_part(&primdrive, true, &target_ip);
    drives_format(&primdrive, true, &target_ip);
    drives_mount(&primdrive, &target_ip);
    build();
    deploy(&target_ip);
    reboot(&target_ip);
    // -----------------------------------------------------
    files_crylia_finish();
    git_full(String::from("Xanterella Remote-Install cleanup"));
}

pub fn clean() {
    files_crylia_finish();
    git_full(String::from("Xanterella Remote-Install cleanup"));
}
