use crate::types::DatabaseType;
use crate::utils::{errors::Error, file::FileUtils, template::TemplateUtils};

pub struct MigrationGenerator;

impl MigrationGenerator {
    pub fn generate(server_dir: &str, database: &DatabaseType) -> Result<(), Error> {
        let template_path = "src/templates/server/migration.toml";
        let template = TemplateUtils::load_template(template_path)?;
        let content = TemplateUtils::replace_placeholders(
            &template,
            &[("database", &format!("{:?}", database))],
        );

        let file_path = format!("{}/migration.rs", server_dir);
        FileUtils::write_file(&file_path, &content)?;
        println!("Migration générée avec succès !");
        Ok(())
    }
}
