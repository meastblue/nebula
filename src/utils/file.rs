use super::errors::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn create_dir_if_not_exists(dir: &str) -> Result<(), Error> {
    if !Path::new(dir).exists() {
        fs::create_dir_all(dir).map_err(Error::FileSystem)?;
    }
    Ok(())
}

pub fn write_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = File::create(path).map_err(Error::FileSystem)?;
    file.write_all(content.as_bytes())
        .map_err(Error::FileSystem)?;
    Ok(())
}

pub fn load_template(template_path: &str) -> Result<String, Error> {
    fs::read_to_string(template_path).map_err(Error::FileSystem)
}

pub fn create_file_in_dir(dir: &str, filename: &str, content: &str) -> Result<(), Error> {
    create_dir_if_not_exists(dir)?;
    let file_path = Path::new(dir).join(filename);
    write_file(file_path.to_str().unwrap(), content)?;
    Ok(())
}

pub fn init_file_from_template(
    project_path: &Path,
    file_path: &str,
    template_path: &str,
    replacements: Option<&[(&str, &str)]>,
) -> Result<(), Error> {
    // Get the executable path
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| Error::InvalidPath)?;
    
    let template_path: PathBuf = if Path::new(template_path).is_absolute() {
        PathBuf::from(template_path)
    } else {
        // Look for templates relative to project root instead of executable
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        project_root.join("src/templates").join(template_path)
    };

    println!("🔍 Loading template from: {:?}", template_path);

    let content = load_template(template_path.to_str().ok_or_else(|| Error::InvalidPath)?)?;

    let content = if let Some(r) = replacements {
        r.iter()
            .fold(content, |acc, (key, value)| acc.replace(key, value))
    } else {
        content
    };

    let full_path = project_path.join(file_path);
    if full_path.exists() {
        fs::remove_file(&full_path)?;
    }

    fs::write(&full_path, content)?;
    Ok(())
}

pub fn copy_directory(src: &Path, dst: &Path) -> Result<(), Error> {
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(Error::FileSystem)?;
    }

    for entry in fs::read_dir(src).map_err(Error::FileSystem)? {
        let entry = entry.map_err(Error::FileSystem)?;
        let ty = entry.file_type().map_err(Error::FileSystem)?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_directory(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(Error::FileSystem)?;
        }
    }
    Ok(())
}

pub fn ensure_directory(path: &Path) -> Result<(), Error> {
    if !path.exists() {
        fs::create_dir_all(path).map_err(Error::FileSystem)?;
    }
    Ok(())
}

pub fn remove_if_exists(path: &Path) -> Result<(), Error> {
    if path.exists() {
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(Error::FileSystem)?;
        } else {
            fs::remove_file(path).map_err(Error::FileSystem)?;
        }
    }
    Ok(())
}
