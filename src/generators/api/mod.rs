pub mod entity;
pub mod handler;

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::template;
use crate::utils::errors::Error;

pub struct ApiGenerator {
    api_dir: String,
}

impl ApiGenerator {
    pub fn new(api_dir: &str) -> Self {
        Self {
            api_dir: api_dir.to_owned(),
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
        Command::new("cargo")
            .arg("init")
            .arg(&self.api_dir)
            .status()
            .map_err(|e| Error::FileSystem(e))
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(Error::Prompt(
                        "Échec de l'initialisation du projet Cargo".into(),
                    ))
                }
            })
    }

    fn init_cargo(&self) -> Result<(), Error> {
        let project_path = Path::new(&self.api_dir);
        let project_name = project_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::InvalidPath)?;

        let content = template::get_cargo_template(project_name);
        let cargo_path = project_path.join("Cargo.toml");
        fs::write(&cargo_path, content).map_err(Error::FileSystem)?;

        println!("✅ Generated Cargo.toml file");
        Ok(())
    }

    fn init_main(&self) -> Result<(), Error> {
        let project_dir = Path::new(&self.api_dir);
        // Generate content from template
        let content = template::get_main_template();

        // Create src/main.rs file
        let main_path = project_dir.join("src/main.rs");
        fs::write(&main_path, content).map_err(Error::FileSystem)?;

        println!("✅ Generated main.rs file");
        Ok(())
    }

    fn init_server(&self) -> Result<(), Error> {
        let project_dir = Path::new(&self.api_dir);
        // Generate content from template
        let content = template::get_server_template();

        // Create src/server.rs file
        let server_path = project_dir.join("src/server.rs");
        fs::write(&server_path, content).map_err(Error::FileSystem)?;

        println!("✅ Generated server.rs file");
        Ok(())
    }

    fn init_route(&self) -> Result<(), Error> {
        let project_dir = Path::new(&self.api_dir);
        // Generate content from template
        let content = template::get_route_template();

        // Create src/route.rs file
        let route_path = project_dir.join("src/route.rs");
        fs::write(&route_path, content).map_err(Error::FileSystem)?;

        println!("✅ Generated route.rs file");
        Ok(())
    }
}
