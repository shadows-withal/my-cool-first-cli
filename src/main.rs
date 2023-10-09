mod say;

use clap::{Parser, Subcommand};
use crate::say::SayCommand;

/// A simple program that prints a message, but it's cute!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Say(SayCommand),
}

fn main() {
    let args = Arguments::parse();

    match args.cmd {
        Commands::Say(cmd) => cmd.run(),
    }
}
