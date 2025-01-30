use crate::utils::{errors::Error, file::FileUtils, template::TemplateUtils};

pub struct HandlerGenerator;

impl HandlerGenerator {
    pub fn generate(server_dir: &str) -> Result<(), Error> {
        let template_path = "src/templates/server/handler.toml";
        let template = TemplateUtils::load_template(template_path)?;
        let content = TemplateUtils::replace_placeholders(&template, &[]);

        let file_path = format!("{}/handler.rs", server_dir);
        FileUtils::write_file(&file_path, &content)?;
        println!("Handler généré avec succès !");
        Ok(())
    }
}
