// use crate::utils::{errors::Error, file, template};

// pub struct HandlerGenerator;

// impl HandlerGenerator {
//     pub fn generate(api_dir: &str) -> Result<(), Error> {
//         let template_path = "src/templates/api/handler.toml";
//         let template = template::load_template(template_path)?;
//         let content = template::replace_placeholders(&template, &[]);

//         let file_path = format!("{}/handler.rs", api_dir);
//         file::write_file(&file_path, &content)?;
//         println!("Handler généré avec succès !");
//         Ok(())
//     }
// }
