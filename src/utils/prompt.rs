use std::str::FromStr;

use super::errors::Error;
use crate::types::{DatabaseType, ProjectType, ServerType};
use inquire::{Select, Text};

pub struct Prompt;

impl Prompt {
    /// Demande à l'utilisateur de choisir un type de projet
    pub fn ask_project_type() -> Result<ProjectType, Error> {
        let options = vec!["client", "server", "fullstack"];
        let choice = Select::new("Quel type de projet voulez-vous créer ?", options)
            .prompt()
            .map_err(|e| Error::InquireError(e))?;
        ProjectType::from_str(&choice)
    }

    /// Demande à l'utilisateur de choisir un type de base de données
    pub fn ask_database_type() -> Result<Option<DatabaseType>, Error> {
        let options = vec!["mysql", "postgresql", "mariadb", "mongodb", "aucune"];
        let choice = Select::new(
            "Quel type de base de données voulez-vous utiliser ?",
            options,
        )
        .prompt()
        .map_err(|e| Error::InquireError(e))?;

        if choice == "aucune" {
            Ok(None)
        } else {
            Ok(Some(DatabaseType::from_str(&choice)?))
        }
    }

    /// Demande à l'utilisateur de choisir un type de serveur
    pub fn ask_server_type() -> Result<Option<ServerType>, Error> {
        let options = vec!["rest", "graphql", "aucun"];
        let choice = Select::new("Quel type de serveur voulez-vous utiliser ?", options)
            .prompt()
            .map_err(|e| Error::InquireError(e))?;

        if choice == "aucun" {
            Ok(None)
        } else {
            Ok(Some(ServerType::from_str(&choice)?))
        }
    }

    /// Demande à l'utilisateur le nom du projet
    pub fn ask_project_name() -> Result<String, Error> {
        Text::new("Nom du projet :")
            .prompt()
            .map_err(|e| Error::InquireError(e))
    }
}
