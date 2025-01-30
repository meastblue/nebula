use crate::cli::GenerateArgs;
use crate::generators::server::{entity, handler, migration, resolver, routes};
use crate::types::FileType;
use colored::*;

/// Exécute la commande `generate`
pub fn run(args: GenerateArgs) {
    match args.file_type {
        FileType::Entity => {
            if let Some(database) = args.database {
                entity::generate(&args.name, &database);
            } else {
                println!(
                    "{}",
                    "Erreur : Le type de base de données est requis pour générer une entité.".red()
                );
            }
        }
        FileType::Handler => {
            handler::generate(&args.name);
        }
        FileType::Migration => {
            if let Some(database) = args.database {
                migration::generate(&args.name, &database);
            } else {
                println!(
                    "{}",
                    "Erreur : Le type de base de données est requis pour générer une migration."
                        .red()
                );
            }
        }
        FileType::Resolver => {
            resolver::generate(&args.name);
        }
        FileType::Routes => {
            routes::generate(&args.name);
        }
    }
}
