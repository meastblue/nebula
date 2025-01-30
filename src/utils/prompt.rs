use std::str::FromStr;

use crate::types::{DatabaseType, ServerType};
use colored::*;
use inquire::{Select, Text};

pub fn ask_project_type() -> String {
    let options = vec!["client", "server", "fullstack"];
    Select::new("Quel type de projet voulez-vous créer ?", options)
        .prompt()
        .unwrap_or_else(|_| {
            println!("{}", "Erreur : Choix invalide.".red());
            std::process::exit(1);
        })
        .to_string()
}

pub fn ask_database_type() -> Option<DatabaseType> {
    let options = vec!["mysql", "postgresql", "mariadb", "mongodb", "aucune"];
    let choice = Select::new(
        "Quel type de base de données voulez-vous utiliser ?",
        options,
    )
    .prompt()
    .unwrap_or_else(|_| {
        println!("{}", "Erreur : Choix invalide.".red());
        std::process::exit(1);
    })
    .to_string();

    if choice == "aucune" {
        None
    } else {
        Some(DatabaseType::from_str(&choice).unwrap_or_else(|_| {
            println!("{}", "Erreur : Type de base de données invalide.".red());
            std::process::exit(1);
        }))
    }
}

pub fn ask_server_type() -> Option<ServerType> {
    let options = vec!["rest", "graphql"];
    let choice = Select::new("Quel type de serveur voulez-vous utiliser ?", options)
        .prompt()
        .unwrap_or_else(|_| {
            println!("{}", "Erreur : Choix invalide.".red());
            std::process::exit(1);
        })
        .to_string();

    if choice == "aucun" {
        None
    } else {
        Some(ServerType::from_str(&choice).unwrap_or_else(|_| {
            println!("{}", "Erreur : Type de serveur invalide.".red());
            std::process::exit(1);
        }))
    }
}

pub fn ask_project_name() -> String {
    Text::new("Nom du projet :").prompt().unwrap_or_else(|_| {
        println!("{}", "Erreur : Nom de projet invalide.".red());
        std::process::exit(1);
    })
}
