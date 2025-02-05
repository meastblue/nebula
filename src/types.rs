use crate::utils::errors::Error;
use clap::ValueEnum;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ProjectType {
    #[clap(alias = "web")]
    Web,

    #[clap(alias = "api")]
    Api,

    #[clap(alias = "full")]
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum FileType {
    Entity,
    Handler,
    Migration,
    Resolver,
    Routes,
}

impl FromStr for ProjectType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "web" => Ok(ProjectType::Web),
            "api" => Ok(ProjectType::Api),
            "full" => Ok(ProjectType::Full),
            _ => Err(Error::InvalidOptions(format!(
                "Type de projet invalide : {}",
                s
            ))),
        }
    }
}

impl ProjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectType::Web => "web",
            ProjectType::Api => "api",
            ProjectType::Full => "full",
        }
    }
}

impl FromStr for FileType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "entity" => Ok(FileType::Entity),
            "handler" => Ok(FileType::Handler),
            "migration" => Ok(FileType::Migration),
            "resolver" => Ok(FileType::Resolver),
            "routes" => Ok(FileType::Routes),
            _ => Err(Error::InvalidOptions(format!(
                "Type de fichier invalide : {}",
                s
            ))),
        }
    }
}
