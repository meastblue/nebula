use crate::types::{DatabaseType, FileType, ProjectType, ServerType};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nebula")]
#[command(
    about = "Un CLI pour générer des projets et fichiers en Rust.",
    version = "1.0.0"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New(NewArgs),
    Generate(GenerateArgs),
}

#[derive(Parser)]
pub struct NewArgs {
    pub name: String,

    #[arg(short, long, value_parser = clap::value_parser!(ProjectType))]
    pub project_type: Option<ProjectType>,

    #[arg(short = 'd', long, value_parser = clap::value_parser!(DatabaseType))]
    pub database: Option<DatabaseType>,

    #[arg(short, long, value_parser = clap::value_parser!(ServerType))]
    pub server_type: Option<ServerType>,
}

#[derive(Parser)]
pub struct GenerateArgs {
    /// Type de fichier à générer (entity, handler, migration, etc.)
    #[arg(value_parser = clap::value_parser!(FileType))]
    pub file_type: FileType,

    /// Nom du fichier
    pub name: String,

    /// Type de base de données (mysql, postgresql, mariadb, mongodb)
    #[arg(short = 'd', long, value_parser = clap::value_parser!(DatabaseType))]
    pub database: Option<DatabaseType>,
}
