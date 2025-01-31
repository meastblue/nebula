use crate::cli::NewArgs;
use crate::generators::{client::ClientGenerator, server::ServerGenerator};
use crate::types::{DatabaseType, ProjectType, ServerType};
use crate::utils::errors::Error;
use crate::utils::{file, prompt};
use std::process::Command;

pub struct NewCommand;

impl NewCommand {
    pub fn run(args: NewArgs) -> Result<(), Error> {
        let project_name = args.name;

        let project_type = match args.project_type {
            Some(project_type) => project_type,
            None => prompt::ask_project_type()?,
        };

        let database = if project_type != ProjectType::Client {
            match args.database {
                Some(database) => Some(database),
                None => prompt::ask_database_type()?,
            }
        } else {
            None
        };

        let server_type = if project_type != ProjectType::Client {
            match args.server_type {
                Some(server_type) => Some(server_type),
                None => prompt::ask_server_type()?,
            }
        } else {
            None
        };

        let project_dir = format!("./{}", project_name);

        match project_type {
            ProjectType::Client => {
                ClientGenerator::generate(&project_dir)?;
            }
            ProjectType::Server => {
                Self::init_cargo_project(&project_dir)?;
                ServerGenerator::new(project_dir, database, server_type).generate()?;
            }
            ProjectType::Fullstack => {
                let client_dir = format!("{}/client", project_dir);
                let server_dir = format!("{}/server", project_dir);

                file::create_dir_if_not_exists(&project_dir)?;
                file::create_dir_if_not_exists(&client_dir)?;
                Self::init_cargo_project(&server_dir)?;

                ClientGenerator::generate(&client_dir)?;
                // ServerGenerator::generate(&server_dir, &database, &server_type)?;
            }
        }

        println!("Projet créé avec succès !");
        Ok(())
    }

    fn init_cargo_project(dir: &str) -> Result<(), Error> {
        let status = Command::new("cargo")
            .arg("new")
            .arg(dir)
            .status()
            .map_err(|e| Error::FileSystem(e))?;

        if !status.success() {
            return Err(Error::Prompt(
                "Échec de l'initialisation du projet Cargo".to_string(),
            ));
        }

        Ok(())
    }
}
