use std::str::FromStr;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ProjectType {
    Client,
    Server,
    Fullstack,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum DatabaseType {
    MySQL,
    PostgreSQL,
    MariaDB,
    MongoDB,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ServerType {
    Rest,
    GraphQL,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum FileType {
    Entity,
    Handler,
    Migration,
    Resolver,
    Routes,
}

impl FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "client" => Ok(ProjectType::Client),
            "server" => Ok(ProjectType::Server),
            "fullstack" => Ok(ProjectType::Fullstack),
            _ => Err(format!("Type de projet invalide : {}", s)),
        }
    }
}

impl FromStr for DatabaseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mysql" => Ok(DatabaseType::MySQL),
            "postgresql" => Ok(DatabaseType::PostgreSQL),
            "mariadb" => Ok(DatabaseType::MariaDB),
            "mongodb" => Ok(DatabaseType::MongoDB),
            _ => Err(format!("Type de base de données invalide : {}", s)),
        }
    }
}

impl FromStr for ServerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rest" => Ok(ServerType::Rest),
            "graphql" => Ok(ServerType::GraphQL),
            _ => Err(format!("Type de serveur invalide : {}", s)),
        }
    }
}

impl FromStr for FileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "entity" => Ok(FileType::Entity),
            "handler" => Ok(FileType::Handler),
            "migration" => Ok(FileType::Migration),
            "resolver" => Ok(FileType::Resolver),
            "routes" => Ok(FileType::Routes),
            _ => Err(format!("Type de fichier invalide : {}", s)),
        }
    }
}
