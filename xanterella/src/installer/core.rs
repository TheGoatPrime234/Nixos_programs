use log::info;

use crate::installer::install::*;
use crate::installer::file::*;
use crate::installer::drives::*;
use crate::utils::git::git_full;
use crate::utils::check::*;
use crate::utils::core::*;
use crate::utils::select::*;
use crate::utils::get::*;

pub fn remote_install(automate: &bool, fast: &bool) {
    let target_ip = select_host(get_taildevices());
    ping_full(&target_ip);
    if *fast {
        let primdrive = select_drive(&target_ip, *automate);
        drives_part(&primdrive, true, &target_ip);
        drives_format(&primdrive, true, &target_ip);
        crylia_edit_start(get_hardware(&target_ip));
        git_full(String::from("Xanterella Remote-Install (fast)"));
        drives_mount(&primdrive, &target_ip);
        build();
        deploy(&target_ip);
        reboot(&target_ip, false);
    } else {
        crylia_edit_start(get_hardware(&target_ip));
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
        logout_tailscale(&target_ip, false);
        reboot(&target_ip, false);
    }
    // -----------------------------------------------------
    clean();
}

pub fn clean() {
    info!("[ RUN ] - Starte Cleanup");

    crylia_edit_finish();
    git_full(String::from("Xanterella Remote-Install cleanup"));
    info!("[ OK ] - Cleanup erfolgreich");
}

pub fn crylia_edit_start(config: String) {
    info!("[ RUN ] - Starte Crylia Überarbeitung");

    create_hardware(config);
    write_config(edit_config(parse_config(), EditMode::Add));
    files_alejandra();
    info!("[ OK ] - Crylia Überarbeitung erfolgreich");
}

pub fn crylia_edit_finish() {
    info!("[ RUN ] - Starte Crylia Überarbeitung");

    remove_hardware(); 
    write_config(edit_config(parse_config(), EditMode::Remove));
    files_alejandra();
    info!("[ OK ] - Crylia Überarbeitung erfolgreich");
}

pub fn drives_part(primdrive: &str, debug: bool, ip: &str) {
    info!("[ RUN ] - Starte Parittionierung");

    let drive = format!("/dev/{}", primdrive);
    part_efi(&drive, debug, ip);
    part_root(&drive, debug, ip);
    info!("[ OK ] - Parittionierung erfolgreich");
}

pub fn drives_format(primdrive: &str, debug: bool, ip: &str) {
    info!("[ RUN ] - Starte Formatierung");

    format_efi(primdrive, debug, ip);
    format_root(primdrive, debug, ip);
    info!("[ Ok ] - Formatierung erfolgreich");
}

pub fn drives_mount(primdrive: &str, ip: &str) {
    info!("[ RUN ] - Starte Mounting");

    mount_root(primdrive, ip);
    create_boot_dir(ip);
    mount_boot(primdrive, ip);
    info!("[ OK ] - Mounting erfolgreich");
}

pub fn deploy(ip: &str) {
    info!("RUN - Starte Deployment");

    copy(ip);
    profile(ip);
    prep(ip);
    activate(ip);
    bootloader(ip);
    info!("[ OK ] - Deployment erfolgreich");
}
