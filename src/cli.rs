use crate::types::{FileType, ProjectType};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "nebula",
    version = "1.0.0",
    about = "CLI for generating Rust projects and components",
    long_about = r#"
Nebula - Outil de productivité pour projets Rust

Exemples:
  Créer un projet API : nebula new mon-projet --type api
  Générer une entité   : nebula generate entity -e User -f "name:String|required,email:String|unique" -r "posts:has_many:Post"
"#
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
