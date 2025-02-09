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
    #[command(alias = "n")]
    New(NewArgs),
    #[command(alias = "g")]
    Generate {
        #[command(subcommand)]
        opts: GenerateArgs,
    },
}

#[derive(Parser)]
pub struct NewArgs {
    pub name: String,
    #[arg(short = 't', long = "type", value_parser = clap::value_parser!(ProjectType))]
    pub opt: Option<ProjectType>,
}

#[derive(Subcommand)]
pub enum GenerateArgs {
    #[command(alias = "e")]
    Entity {
        name: String,
        #[arg(long = "fields", short = 'f')]
        fields: Option<String>,
        #[arg(long = "relations", short = 'r')]
        relations: Option<String>,
    },
    #[command(alias = "h")]
    Handler { name: String },
}
