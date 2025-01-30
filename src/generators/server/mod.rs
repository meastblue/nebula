pub mod entity;
pub mod handler;
pub mod migration;
pub mod resolver;
pub mod routes;

use crate::types::{DatabaseType, ServerType};
use crate::utils::{errors::Error, file::FileUtils};

pub struct ServerGenerator;

impl ServerGenerator {
    pub fn generate(
        project_dir: &str,
        database: &Option<DatabaseType>,
        server_type: &Option<ServerType>,
    ) -> Result<(), Error> {
        let server_dir = format!("{}/server", project_dir);
        FileUtils::create_dir_if_not_exists(&server_dir)?;

        // Générer les fichiers en fonction des choix de l'utilisateur
        if let Some(database) = database {
            entity::EntityGenerator::generate(&server_dir, database)?;
            migration::MigrationGenerator::generate(&server_dir, database)?;
        }

        if let Some(server_type) = server_type {
            match server_type {
                ServerType::Rest => {
                    handler::HandlerGenerator::generate(&server_dir)?;
                    routes::RoutesGenerator::generate(&server_dir)?;
                }
                ServerType::GraphQL => {
                    resolver::ResolverGenerator::generate(&server_dir)?;
                    routes::RoutesGenerator::generate(&server_dir)?;
                }
            }
        }

        Ok(())
    }
}
