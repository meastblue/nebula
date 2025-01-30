mod cli;
mod commands;
mod generators;
mod types;
mod utils;

use clap::Parser;
use cli::Cli;
use commands::{generate::GenerateCommand, new::NewCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse les arguments de la ligne de commande
    let cli = Cli::parse();

    // Exécute la commande appropriée
    match cli.command {
        cli::Commands::New(args) => NewCommand::run(args)?,
        cli::Commands::Generate(args) => GenerateCommand::run(args)?,
    }

    Ok(())
}
