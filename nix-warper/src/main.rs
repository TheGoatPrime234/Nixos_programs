use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::{fs, env};


#[derive(Parser)]
#[command(name = "Nix-Warper")]
#[command(about = "Schnelles Navigieren im Filesystem mit sogenannten Warppoints", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Warp {
        place: String,
    },
    Set {
        alias: String,
    },
    List,
    Init,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Structure {
    pub alias: String,
    pub dir: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Warp { place } => {
            warp(&place);
        },
        Commands::Set { alias } => {
            set(&alias);
        },
        Commands::List => {
            list();
        },
        Commands::Init => {
            init();
        },
    }
}

fn read() -> Vec<Structure> {
    let file_path = "/home/cato/.config/nix-warper/points.json";
    let content = fs::read_to_string(&file_path).expect("Fehler beim lesen");
    let json: Vec<Structure> = serde_json::from_str(&content).unwrap();
    json
}
    
fn set(alias: &String) {
    let file_path = "/home/cato/.config/nix-warper/points.json";
    let to_append = Structure {
        alias: alias.to_string(),
        dir: env::current_dir().unwrap().to_string_lossy().to_string(),
    };
    let mut content = read();
    content.push(to_append);
    let json_string = serde_json::to_string_pretty(&content).unwrap();
    fs::write(&file_path, &json_string).expect("Konnte die points nicht beschreiben");
}

fn init() {
    let file_path = "/home/cato/.config/nix-warper/points.json";
    let _ = fs::write(file_path, "[]").expect("Konnte die Init-Datei nicht erstellen");
}

fn warp(place: &String) {
    let found = read().into_iter().find(|p| p.alias == *place);
    match found {
        Some(point) => {
            println!("{}", point.dir);
        }
        None => {
            println!("Warppoint nicht gefunden");
        }
    }
}

fn list() {
    for i in read() {
        println!("{:?}", i);
    }
}
