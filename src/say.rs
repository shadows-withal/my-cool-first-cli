use clap::Parser;

#[derive(Debug, Parser)]
/// Make your Axolotl say something!
pub struct SayCommand {
    /// The message to print.
    message: String,
}

impl SayCommand {
    pub fn run(&self) {
        let message = &self.message;
        let dashes = "-".repeat(message.len() + 2);
        println!("         +{dashes}+");
        println!("         | {message} |");
        println!("         +{dashes}+");
        println!("        /");
        println!("≽(◕ ᴗ ◕)≼");
    }
}
