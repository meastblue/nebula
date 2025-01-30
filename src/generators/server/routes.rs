use crate::utils::{errors::Error, file::FileUtils, template::TemplateUtils};

pub struct RoutesGenerator;

impl RoutesGenerator {
    pub fn generate(server_dir: &str) -> Result<(), Error> {
        let template_path = "src/templates/server/routes.toml";
        let template = TemplateUtils::load_template(template_path)?;
        let content = TemplateUtils::replace_placeholders(&template, &[]);

        let file_path = format!("{}/routes.rs", server_dir);
        FileUtils::write_file(&file_path, &content)?;
        println!("Routes générées avec succès !");
        Ok(())
    }
}
