use std::path::Path;

use crate::types::ProjectType;

use super::errors::Error;

pub fn check_is_nebula_project() -> Result<(), Error> {
    if !Path::new("nebula.config.toml").exists() {
        return Err(Error::NotNebulaProject);
    }

    Ok(())
}

pub fn get_project_config() -> Result<ProjectType, Error> {
    let is_src_present = Path::new("src").exists();
    let is_web_present = Path::new("web").exists();
    let is_api_present = Path::new("api").exists();

    match (is_src_present, is_web_present, is_api_present) {
        (true, false, false) => Ok(ProjectType::Api),
        (false, true, true) => Ok(ProjectType::Full),
        (false, true, false) => Ok(ProjectType::Web),
        _ => Err(Error::InvalidProjectStructure),
    }
}
