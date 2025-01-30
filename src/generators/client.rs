use crate::utils::file;

use colored::*;

pub fn generate(project_dir: &str) {
    // Exemple : Créer un fichier de composant
    let component = r#"
    // Composant généré par Nebula
    fn main() {
        println!("Hello, world!");
    }
    "#;

    let client_dir = format!("{}/client", project_dir);
    if let Err(err) = file::create_file_in_dir(&client_dir, "component.rs", component) {
        println!("{}", err.red());
        return;
    }

    println!("{}", "Client généré avec succès !".green());
}
