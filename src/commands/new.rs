use crate::cli::NewArgs;
use crate::generators::{client::ClientGenerator, server::ServerGenerator};
use crate::types::ProjectType;
use crate::utils::{errors::Error, file::FileUtils, prompt::Prompt};

pub struct NewCommand;

impl NewCommand {
    pub fn run(args: NewArgs) -> Result<(), Error> {
        // Utilise le nom du projet fourni en ligne de commande
        let project_name = args.name;

        // Demande le type de projet si non fourni
        let project_type = match args.project_type {
            Some(project_type) => project_type,
            None => Prompt::ask_project_type()?,
        };

        // Demande le type de base de données si non fourni et si applicable
        let database = if project_type != ProjectType::Client {
            match args.database {
                Some(database) => Some(database),
                None => Prompt::ask_database_type()?,
            }
        } else {
            None
        };

        // Demande le type de serveur si non fourni et si applicable
        let server_type = if project_type != ProjectType::Client {
            match args.server_type {
                Some(server_type) => Some(server_type),
                None => Prompt::ask_server_type()?,
            }
        } else {
            None
        };

        // Crée le dossier du projet
        let project_dir = format!("./{}", project_name);
        FileUtils::create_dir_if_not_exists(&project_dir)?;

        // Génère les fichiers en fonction du type de projet
        match project_type {
            ProjectType::Client => ClientGenerator::generate(&project_dir)?,
            ProjectType::Server => {
                ServerGenerator::generate(&project_dir, &database, &server_type)?
            }
            ProjectType::Fullstack => {
                ClientGenerator::generate(&project_dir)?;
                ServerGenerator::generate(&project_dir, &database, &server_type)?;
            }
        }

        println!("Projet créé avec succès !");
        Ok(())
    }
}
