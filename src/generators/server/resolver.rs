use crate::utils::{file, template};
use colored::*;

pub fn generate(server_dir: &str) {
    let template_path = "src/templates/server/resolver.toml";

    let template = match template::load_template(template_path) {
        Ok(template) => template,
        Err(err) => {
            println!("{}", err.red());
            return;
        }
    };

    let content = template::replace_placeholders(&template, &[]);

    let file_path = format!("{}/resolver.rs", server_dir);
    if let Err(err) = file::write_file(&file_path, &content) {
        println!("{}", err.red());
        return;
    }

    println!("{}", "Resolver généré avec succès !".green());
}
