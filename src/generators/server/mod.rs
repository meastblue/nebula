pub mod entity;
pub mod handler;
pub mod migration;
pub mod resolver;
pub mod routes;

use std::path::Path;

use crate::types::{DatabaseType, ServerType};
use crate::utils::file::init_file_from_template;
use crate::utils::template;
use crate::utils::{errors::Error, file};

pub struct ServerGenerator {
    project_dir: String,
    database: Option<DatabaseType>,
    server_type: Option<ServerType>,
}

impl ServerGenerator {
    pub fn new(
        project_dir: String,
        database: Option<DatabaseType>,
        server_type: Option<ServerType>,
    ) -> Self {
        Self {
            project_dir: project_dir.to_owned(),
            database,
            server_type,
        }
    }

    pub fn generate(&self) -> Result<(), Error> {
        self.init_cargo()?;
        self.init_main()?;
        self.init_server()?;
        self.init_route()?;
        // if let Some(database) = database {
        //     entity::EntityGenerator::generate(project_dir, database)?;
        //     migration::MigrationGenerator::generate(project_dir, database)?;
        // }

        // if let Some(server_type) = server_type {
        //     match server_type {
        //         ServerType::Rest => {
        //             handler::HandlerGenerator::generate(project_dir)?;
        //             routes::RoutesGenerator::generate(project_dir)?;
        //         }
        //         ServerType::GraphQL => {
        //             resolver::ResolverGenerator::generate(project_dir)?;
        //             routes::RoutesGenerator::generate(project_dir)?;
        //         }
        //     }
        // }

        Ok(())
    }

    fn init_cargo(&self) -> Result<(), Error> {
        let project_dir = Path::new(&self.project_dir);
        let template_path = "server/cargo.toml";
        let file_path = "cargo.toml";

        init_file_from_template(project_dir, file_path, template_path, None)?;
        Ok(())
    }

    fn init_main(&self) -> Result<(), Error> {
        let project_dir = Path::new(&self.project_dir);
        let template_path = "server/main.toml";
        let file_path = "src/main.rs";

        init_file_from_template(project_dir, file_path, template_path, None)?;
        Ok(())
    }

    fn init_server(&self) -> Result<(), Error> {
        let project_dir = Path::new(&self.project_dir);
        let template_path = "server/server.toml";
        let file_path = "src/server.rs";

        init_file_from_template(project_dir, file_path, template_path, None)?;
        Ok(())
    }

    fn init_route(&self) -> Result<(), Error> {
        let project_dir = Path::new(&self.project_dir);
        let template_path = "server/route.toml";
        let file_path = "src/route.rs";

        init_file_from_template(project_dir, file_path, template_path, None)?;
        Ok(())
    }
}
