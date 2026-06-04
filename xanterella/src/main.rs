mod cli;
mod installer;
mod utils;

use cli::commands::cli_parse;

fn main() {
    cli_parse();
}
