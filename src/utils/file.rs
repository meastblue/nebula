use toml::Value;

use super::errors::Error;
use super::template::load_template;
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_dir_if_not_exists(dir: &str) -> Result<(), Error> {
    if Path::new(dir).exists() {
        return Err(Error::ElementAlreadyExists(dir.to_string()));
    }
    fs::create_dir_all(dir)?;
    Ok(())
}

pub fn write_file(path: &str, content: &str) -> Result<(), Error> {
    fs::write(path, content).map_err(|e| {
        Error::FileSystem(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write file '{}': {}", path, e),
        ))
    })?;
    Ok(())
}

pub fn create_file_in_dir(dir: &str, filename: &str, content: &str) -> Result<(), Error> {
    create_dir_if_not_exists(dir)?;
    let file_path = format!("{}/{}", dir, filename);
    write_file(&file_path, content)?;
    Ok(())
}

pub fn init_file_from_template(
    project_path: &Path,
    file_path: &str,
    template_path: &str,
    replacements: Option<&[(&str, &str)]>,
) -> Result<(), Error> {
    let template_path: PathBuf = if Path::new(template_path).is_absolute() {
        PathBuf::from(template_path)
    } else {
        PathBuf::from("src/templates").join(template_path)
    };

    println!("🔍 Utilisation du template: {:?}", template_path);

    let mut content = load_template(template_path.to_str().unwrap())?;

    if let Some(r) = replacements {
        content = r
            .iter()
            .fold(content, |acc, (key, value)| acc.replace(key, value));
    }

    let full_path = project_path.join(file_path);

    if full_path.exists() {
        fs::remove_file(&full_path)?;
    }

    fs::write(&full_path, content)?;
    Ok(())
}
