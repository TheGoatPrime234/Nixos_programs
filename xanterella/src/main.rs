mod check;
mod files;
mod generator;
mod get;
mod git;
mod list;
mod select;
mod drives;

use check::*;
use files::*;
use get::*;
use git::*;
use list::*;
use select::*;
use drives::*;

use std::process::Command;
use clap::{Parser, Subcommand};
use inquire::Select;

#[derive(Parser)]
#[command(name = "Xanterella")]
#[command(about = "Verwaltung der Nix & Nixos Configuration von Xanterella für einen und mehrere Hosts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Hostname,
    Ping {
        ip: String,
    },
    Clean,
    Debug {
        #[arg(value_enum)]
        option: ListDebug,
    },
    RemoteInstall {
        #[arg(long = "automate", short = 'a')]
        automate: bool,
        #[arg(long = "fast", short = 'f')]
        fast: bool,
    },
}

pub fn main() {
    let cli = Cli::parse();
    let log_level = if cli.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder()
        .filter_level(log_level)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .init();
    match &cli.command {
        Commands::Hostname => {
            let _ = Command::new("hostname") .spawn();
        },
        Commands::Ping { ip } => {
            ssh_ping(ip);
        },
        Commands::Clean => {
            files_crylia_finish();
            git_full(String::from("Xanterella Remote-Install cleanup"));
        },
        Commands::Debug { option } => {
            list_debug(&option);
        },
        Commands::RemoteInstall { automate, fast } => {
            remote_install(&automate, &fast);
        },
    }
}

pub fn remote_install(automate: &bool, fast: &bool) {
    let target_ip = select_host(get_taildevices());
    ssh_ping(&target_ip);
    get_ssh_hardware(&target_ip);
    files_crylia_start(get_ssh_hardware(&target_ip));
    git_full(String::from("Xanterella Remote-Install"));
    if *fast {
        nix_check();
    };
    let primdrive = select_drive(&target_ip, *automate);
    drives_part(&primdrive, true, &target_ip);
    drives_mount(&primdrive, &target_ip);
    build_and_deploy(&target_ip);
    // -----------------------------------------------------
    files_crylia_finish();
    git_full(String::from("Xanterella Remote-Install cleanup"));
}
