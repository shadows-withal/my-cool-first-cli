use anyhow::anyhow;
use clap::Parser;

const MAX_MESSAGE_LENGTH: usize = 50;

#[derive(Debug, Parser)]
/// Make your Axolotl say something!
pub struct SayCommand {
    /// The message to print.
    message: String,
}

impl SayCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let message = &self.message;

        // Error if the user input a message that's too long!
        if message.len() > MAX_MESSAGE_LENGTH {
            return Err(anyhow!("Messages can't be longer than {MAX_MESSAGE_LENGTH} characters!"));
        }

        let dashes = "-".repeat(message.len() + 2);
        println!("         +{dashes}+");
        println!("         | {message} |");
        println!("         +{dashes}+");
        println!("        /");
        println!("≽(◕ ᴗ ◕)≼");

        Ok(())
    }
}
