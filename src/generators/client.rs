use crate::utils::{errors::Error, file::FileUtils};

pub struct ClientGenerator;

impl ClientGenerator {
    pub fn generate(project_dir: &str) -> Result<(), Error> {
        let client_dir = format!("{}/client", project_dir);
        let component = r#"
        // Composant généré par Nebula
        fn main() {
            println!("Hello, world!");
        }
        "#;

        FileUtils::create_file_in_dir(&client_dir, "component.rs", component)?;
        println!("Client généré avec succès !");
        Ok(())
    }
}
