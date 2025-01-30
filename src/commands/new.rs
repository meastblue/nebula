use std::str::FromStr;

use crate::cli::NewArgs;
use crate::generators::{client, server};
use crate::types::{DatabaseType, ProjectType, ServerType};
use crate::utils::{file, prompt};
use colored::*;

pub fn run(args: NewArgs) {
    let project_name = args.name;

    let project_type = match args.project_type {
        Some(project_type) => project_type,
        None => ProjectType::from_str(&prompt::ask_project_type()).unwrap_or_else(|_| {
            println!("{}", "Erreur : Type de projet invalide.".red());
            std::process::exit(1);
        }),
    };

    let database = if project_type != ProjectType::Client {
        match args.database {
            Some(database) => Some(database),
            None => prompt::ask_database_type(),
        }
    } else {
        None
    };

    let server_type = if project_type != ProjectType::Client {
        match args.server_type {
            Some(server_type) => Some(server_type),
            None => prompt::ask_server_type(),
        }
    } else {
        None
    };

    let project_dir = format!("./{}", project_name);
    if let Err(err) = file::create_dir_if_not_exists(&project_dir) {
        println!("{}", err.red());
        return;
    }

    match project_type {
        ProjectType::Client => client::generate(&project_dir),
        ProjectType::Server => server::generate(&project_dir, &database, &server_type),
        ProjectType::Fullstack => {
            client::generate(&project_dir);
            server::generate(&project_dir, &database, &server_type);
        }
    }

    println!("{}", "Projet créé avec succès !".green());
}
