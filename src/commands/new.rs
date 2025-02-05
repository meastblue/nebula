use std::fs;
use std::path::Path;

use crate::cli::NewArgs;
use crate::generators::{api::ApiGenerator, web::WebGenerator};
use crate::template;
use crate::types::ProjectType;
use crate::utils::errors::Error;
use crate::utils::file::init_file_from_template;
use crate::utils::{file, prompt};

pub struct NewCommand;

impl NewCommand {
    pub fn run(args: NewArgs) -> Result<(), Error> {
        let project_name = args.name;

        let project_type = match args.opt {
            Some(opt) => opt,
            None => prompt::ask_project_type()?,
        };

        let project_dir = format!("./{}", &project_name);

        match project_type {
            ProjectType::Web => {
                WebGenerator::generate(&project_dir)?;
            }
            ProjectType::Api => {
                ApiGenerator::new(&project_dir).generate()?;
            }
            ProjectType::Full => {
                let web_dir = format!("{}/web", &project_dir);
                let api_dir = format!("{}/api", project_dir);

                file::create_dir_if_not_exists(&project_dir)?;
                file::create_dir_if_not_exists(&api_dir)?;
                file::create_dir_if_not_exists(&web_dir)?;

                ApiGenerator::new(&api_dir).generate()?;
            }
        }

        Self::generate_env(&project_dir)?;
        Self::generate_gitignore(&project_dir)?;
        Self::generate_nebula_config(&project_dir, project_type)?;
        Self::generate_readme(&project_dir, &project_name)?;

        println!("Projet créé avec succès !");
        Ok(())
    }

    fn generate_env(dir: &str) -> Result<(), Error> {
        let project_path = Path::new(dir);
        let project_name = project_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::InvalidPath)?;
        let content = template::get_env_template(project_name);
        let env_path = project_path.join(".env");

        fs::write(&env_path, &content).map_err(Error::FileSystem)?;

        println!("✅ Generated environment files");
        Ok(())
    }

    fn generate_gitignore(dir: &str) -> Result<(), Error> {
        let project_path = Path::new(dir);
        let content = template::get_gitignore_template();
        let env_path = project_path.join(".env");

        fs::write(&env_path, &content).map_err(Error::FileSystem)?;

        println!("✅ Generated environment files");
        Ok(())
    }

    fn generate_nebula_config(dir: &str, project_type: ProjectType) -> Result<(), Error> {
        let project_path = Path::new(dir);
        let project_name = project_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::InvalidPath)?;

        let content = template::get_nebula_template(
            project_name,
            project_type.as_str(),
            "postgresql",
            "rest",
        );

        let config_path = project_path.join("nebula.config.toml");
        fs::write(&config_path, &content).map_err(Error::FileSystem)?;

        println!("✅ Generated nebula configuration file");
        Ok(())
    }

    fn generate_readme(dir: &str, project_name: &str) -> Result<(), Error> {
        let project_path = Path::new(dir);
        let content = template::get_readme_template(project_name);
        let readme_path = project_path.join("README.md");

        fs::write(&readme_path, &content).map_err(Error::FileSystem)?;

        println!("✅ Generated README file");
        Ok(())
    }
}
