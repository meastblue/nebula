use crate::cli::GenerateArgs;
use crate::generators::server::{
    entity::EntityGenerator, handler::HandlerGenerator, migration::MigrationGenerator,
    resolver::ResolverGenerator, routes::RoutesGenerator,
};
use crate::types::FileType;
use crate::utils::errors::Error;
use colored::*;

pub struct GenerateCommand;

impl GenerateCommand {
    pub fn run(args: GenerateArgs) -> Result<(), Error> {
        match args.file_type {
            FileType::Entity => {
                if let Some(database) = args.database {
                    EntityGenerator::generate(&args.name, &database)
                } else {
                    println!(
                        "{}",
                        "Erreur : Le type de base de données est requis pour générer une entité."
                            .red()
                    );
                    Ok(())
                }
            }
            FileType::Handler => HandlerGenerator::generate(&args.name),
            FileType::Migration => {
                if let Some(database) = args.database {
                    MigrationGenerator::generate(&args.name, &database)
                } else {
                    println!(
                        "{}",
                        "Erreur : Le type de base de données est requis pour générer une migration."
                            .red()
                    );
                    Ok(())
                }
            }
            FileType::Resolver => ResolverGenerator::generate(&args.name),
            FileType::Routes => RoutesGenerator::generate(&args.name),
        }
    }
}
