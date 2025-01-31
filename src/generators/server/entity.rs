use crate::types::DatabaseType;
use crate::utils::{errors::Error, file, template};

pub struct EntityGenerator;

impl EntityGenerator {
    pub fn generate(server_dir: &str, database: &DatabaseType) -> Result<(), Error> {
        let template_path = "src/templates/server/entity.toml";
        let template = template::load_template(template_path)?;
        let content =
            template::replace_placeholders(&template, &[("database", &format!("{:?}", database))]);

        let file_path = format!("{}/entity.rs", server_dir);
        file::write_file(&file_path, &content)?;
        println!("Entité générée avec succès !");
        Ok(())
    }
}
