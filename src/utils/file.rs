use super::errors::Error;
use std::fs;
use std::path::Path;

pub struct FileUtils;

impl FileUtils {
    /// Crée un répertoire s'il n'existe pas déjà
    pub fn create_dir_if_not_exists(dir: &str) -> Result<(), Error> {
        if Path::new(dir).exists() {
            return Err(Error::ElementAlreadyExists(dir.to_string()));
        }
        fs::create_dir_all(dir)?;
        Ok(())
    }

    /// Écrit du contenu dans un fichier
    pub fn write_file(path: &str, content: &str) -> Result<(), Error> {
        fs::write(path, content)?;
        Ok(())
    }

    /// Crée un dossier (s'il n'existe pas) et y écrit un fichier
    pub fn create_file_in_dir(dir: &str, filename: &str, content: &str) -> Result<(), Error> {
        Self::create_dir_if_not_exists(dir)?;
        let file_path = format!("{}/{}", dir, filename);
        Self::write_file(&file_path, content)?;
        Ok(())
    }
}
