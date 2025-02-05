use crate::utils::{errors::Error, file};

pub struct WebGenerator;

impl WebGenerator {
    pub fn generate(project_dir: &str) -> Result<(), Error> {
        let component = r#"
        fn main() {
            println!("Hello, world!");
        }
        "#;

        file::create_file_in_dir(project_dir, "component.rs", component)?;
        println!("Client généré avec succès !");
        Ok(())
    }
}
