use std::fs;
use std::path::Path;
use colored::*;

/// Crée un répertoire s'il n'existe pas déjà
pub fn create_dir_if_not_exists(dir: &str) -> Result<(), String> {
    if Path::new(dir).exists() {
        return Ok(()); // Le dossier existe déjà, pas besoin de le créer
    }
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    Ok(())
}

/// Écrit du contenu dans un fichier
pub fn write_file(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

/// Crée un dossier (s'il n'existe pas) et y écrit un fichier
pub fn create_file_in_dir(dir: &str, filename: &str, content: &str) -> Result<(), String> {
    create_dir_if_not_exists(dir)?; // Crée le dossier s'il n'existe pas
    let file_path = format!("{}/{}", dir, filename);
    write_file(&file_path, content)?;
    Ok(())
}