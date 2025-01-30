pub mod entity;
pub mod handler;
pub mod migration;
pub mod resolver;
pub mod routes;

use colored::*;

use crate::types::{DatabaseType, ServerType};
use crate::utils::file;

pub fn generate(
    project_dir: &str,
    database: &Option<DatabaseType>,
    server_type: &Option<ServerType>,
) {
    let server_dir = format!("{}/server", project_dir);

    if let Err(err) = file::create_dir_if_not_exists(&server_dir) {
        println!("{}", err.red());
        return;
    }

    // Générer les fichiers en fonction des choix de l'utilisateur
    if let Some(database) = database {
        entity::generate(&server_dir, database);
        migration::generate(&server_dir, database);
    }

    if let Some(server_type) = server_type {
        match server_type {
            ServerType::Rest => {
                handler::generate(&server_dir);
                routes::generate(&server_dir);
            }
            ServerType::GraphQL => {
                resolver::generate(&server_dir);
                routes::generate(&server_dir);
            }
        }
    }
}
