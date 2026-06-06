mod cli;
mod installer;
mod usb;
mod utils;

use cli::commands::cli_parse;

fn main() {
    cli_parse();
}
