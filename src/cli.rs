use crate::types::{FileType, ProjectType};
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

    #[arg(short = 't', long = "type", value_parser = clap::value_parser!(ProjectType))]
    pub project_type: Option<ProjectType>,
}

#[derive(Parser)]
pub struct GenerateArgs {
    #[arg(value_parser = clap::value_parser!(FileType))]
    pub file_type: FileType,

    pub name: String,
}
