use clap::Parser;

/// A simple program that prints a message, but it's cute!
#[derive(Parser, Debug)]
struct Arguments {
    message: String,
}

fn main() {
    let args = Arguments::parse();

    println!("{}", args.message);
}
