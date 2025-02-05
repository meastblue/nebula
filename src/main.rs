mod cli;
mod commands;
mod generators;
mod template;
mod types;
mod utils;

use clap::Parser;
use cli::Cli;
use commands::{generate::GenerateCommand, new::NewCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Err(e) = match cli.command {
        cli::Commands::New(args) => NewCommand::run(args),
        cli::Commands::Generate { opts } => GenerateCommand::run(opts),
    } {
        eprintln!("{}", e);
        return Err(Box::new(e));
    }

    Ok(())
}
