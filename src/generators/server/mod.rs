pub mod entity;
pub mod handler;
pub mod migration;
pub mod resolver;
pub mod routes;

use std::path::Path;
use std::process::Command;

use crate::utils::errors::Error;
use crate::utils::file::init_file_from_template;

pub struct ServerGenerator {
    project_dir: String,
}

impl ServerGenerator {
    pub fn new(project_dir: &str) -> Self {
        Self {
            project_dir: project_dir.to_owned(),
        }
    }

    pub fn generate(&self) -> Result<(), Error> {
        self.init_cargo_project()?;
        self.init_cargo()?;
        self.init_main()?;
        self.init_server()?;
        self.init_route()?;

        Ok(())
    }

    fn init_cargo_project(&self) -> Result<(), Error> {
        let status = Command::new("cargo")
            .arg("init")
            .arg(&self.project_dir)
            .status()
            .map_err(|e| Error::FileSystem(e))?;

        if !status.success() {
            return Err(Error::Prompt(
                "Échec de l'initialisation du projet Cargo".to_string(),
            ));
        }

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
