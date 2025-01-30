use crate::utils::{errors::Error, file::FileUtils, template::TemplateUtils};

pub struct ResolverGenerator;

impl ResolverGenerator {
    pub fn generate(server_dir: &str) -> Result<(), Error> {
        let template_path = "src/templates/server/resolver.toml";
        let template = TemplateUtils::load_template(template_path)?;
        let content = TemplateUtils::replace_placeholders(&template, &[]);

        let file_path = format!("{}/resolver.rs", server_dir);
        FileUtils::write_file(&file_path, &content)?;
        println!("Resolver généré avec succès !");
        Ok(())
    }
}
