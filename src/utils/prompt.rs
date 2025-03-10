use super::errors::Error;
use crate::types::ProjectType;
use inquire::{Select, Text};

pub fn ask_project_type() -> Result<ProjectType, Error> {
    const OPTIONS: &[&str] = &["web", "api", "full"];
    Select::new("Quel type de projet voulez-vous créer ?", OPTIONS.to_vec())
        .with_help_message("Choisissez le type de projet à créer")
        .prompt()
        .map_err(Error::InquireError)?
        .parse()  // Assuming ProjectType implements FromStr
}

// pub fn ask_database_type() -> Result<Option<DatabaseType>, Error> {
//     let options = vec!["mysql", "postgresql", "mariadb", "mongodb", "aucune"];
//     let choice = Select::new(
//         "Quel type de base de données voulez-vous utiliser ?",
//         options,
//     )
//     .prompt()
//     .map_err(|e| Error::InquireError(e))?;

//     if choice == "aucune" {
//         Ok(None)
//     } else {
//         Ok(Some(DatabaseType::from_str(&choice)?))
//     }
// }

// pub fn ask_server_type() -> Result<Option<ServerType>, Error> {
//     let options = vec!["rest", "graphql", "aucun"];
//     let choice = Select::new("Quel type de serveur voulez-vous utiliser ?", options)
//         .prompt()
//         .map_err(|e| Error::InquireError(e))?;

//     if choice == "aucun" {
//         Ok(None)
//     } else {
//         Ok(Some(ServerType::from_str(&choice)?))
//     }
// }

pub fn ask_project_name() -> Result<String, Error> {
    Text::new("Nom du projet :")
        .prompt()
        .map_err(|e| Error::InquireError(e))
}
