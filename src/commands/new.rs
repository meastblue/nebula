use std::path::Path;

use crate::cli::NewArgs;
use crate::generators::{client::ClientGenerator, server::ServerGenerator};
use crate::types::ProjectType;
use crate::utils::errors::Error;
use crate::utils::file::init_file_from_template;
use crate::utils::{file, prompt};

pub struct NewCommand;

impl NewCommand {
    pub fn run(args: NewArgs) -> Result<(), Error> {
        let project_name = args.name;

        let project_type = match args.project_type {
            Some(project_type) => project_type,
            None => prompt::ask_project_type()?,
        };

        let project_dir = format!("./{}", &project_name);

        match project_type {
            ProjectType::Client => {
                ClientGenerator::generate(&project_dir)?;
            }
            ProjectType::Server => {
                ServerGenerator::new(&project_dir).generate()?;
            }
            ProjectType::Full => {
                let client_dir = format!("{}/client", &project_dir);
                let server_dir = format!("{}/server", project_dir);

                file::create_dir_if_not_exists(&project_dir)?;
                file::create_dir_if_not_exists(&server_dir)?;
                file::create_dir_if_not_exists(&client_dir)?;

                ServerGenerator::new(&server_dir).generate()?;
            }
        }

        Self::generate_env(&project_dir)?;
        Self::generate_gitignore(&project_dir)?;

        println!("Projet créé avec succès !");
        Ok(())
    }

    fn generate_env(dir: &str) -> Result<(), Error> {
        let project_path = Path::new(dir);
        let template_path = "env.toml";
        let file_path = ".env";

        println!("{:?}", &project_path);

        init_file_from_template(project_path, file_path, template_path, None)?;

        Ok(())
    }

    fn generate_gitignore(dir: &str) -> Result<(), Error> {
        let project_path = Path::new(dir);
        let template_path = "gitignore.toml";
        let file_path = ".gitignore";

        init_file_from_template(project_path, file_path, template_path, None)?;

        Ok(())
    }
}
