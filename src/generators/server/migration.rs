use crate::types::DatabaseType;
use crate::utils::{file, template};
use colored::*;

pub fn generate(server_dir: &str, database: &DatabaseType) {
    let template_path = "src/templates/server/migration.toml";

    let template = match template::load_template(template_path) {
        Ok(template) => template,
        Err(err) => {
            println!("{}", err.red());
            return;
        }
    };

    let content =
        template::replace_placeholders(&template, &[("database", &format!("{:?}", database))]);

    let file_path = format!("{}/migration.rs", server_dir);
    if let Err(err) = file::write_file(&file_path, &content) {
        println!("{}", err.red());
        return;
    }

    println!("{}", "Migration générée avec succès !".green());
}
