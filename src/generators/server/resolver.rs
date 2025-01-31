use crate::utils::{errors::Error, file, template};

pub struct ResolverGenerator;

impl ResolverGenerator {
    pub fn generate(server_dir: &str) -> Result<(), Error> {
        let template_path = "src/templates/server/resolver.toml";
        let template = template::load_template(template_path)?;
        let content = template::replace_placeholders(&template, &[]);

        let file_path = format!("{}/resolver.rs", server_dir);
        file::write_file(&file_path, &content)?;
        println!("Resolver généré avec succès !");
        Ok(())
    }
}
