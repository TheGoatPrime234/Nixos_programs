use std::process::Command;
use clap::{Parser, Subcommand};

use crate::utils::check::ssh_ping;
use crate::utils::git::git_full;
use crate::utils::debug::{list_debug, ListDebug};

use crate::installer::core::remote_install;
use crate::installer::file::files_crylia_finish;

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

pub fn cli_parse() {
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
            let _ = Command::new("hostname").spawn();
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
