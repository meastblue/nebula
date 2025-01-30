mod cli;
mod commands;
mod generators;
mod types;
mod utils;

use clap::Parser;
use cli::Cli;

fn main() {
    // Parse les arguments de la ligne de commande
    let cli = Cli::parse();

    // Exécute la commande appropriée
    match cli.command {
        cli::Commands::New(args) => commands::new::run(args),
        cli::Commands::Generate(args) => commands::generate::run(args),
    }
}
