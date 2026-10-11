mod cli;
mod display;
mod frames;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    if let Err(e) = display::run(&cli) {
        eprintln!("clpcat: {e}");
        std::process::exit(1);
    }
}
