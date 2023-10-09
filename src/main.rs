use clap::Parser;

/// A simple program that prints a message, but it's cute!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// The message to print.
    message: String,
}

fn main() {
    let args = Arguments::parse();

    let message = &args.message;
    let dashes = "-".repeat(message.len() + 2);
    println!("         +{dashes}+");
    println!("         | {message} |");
    println!("         +{dashes}+");
    println!("        /");
    println!("≽(◕ ᴗ ◕)≼");
}
