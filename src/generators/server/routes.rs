use crate::utils::{errors::Error, file, template};

pub struct RoutesGenerator;

impl RoutesGenerator {
    pub fn generate(server_dir: &str) -> Result<(), Error> {
        let template_path = "src/templates/server/routes.toml";
        let template = template::load_template(template_path)?;
        let content = template::replace_placeholders(&template, &[]);

        let file_path = format!("{}/routes.rs", server_dir);
        file::write_file(&file_path, &content)?;
        println!("Routes générées avec succès !");
        Ok(())
    }
}
