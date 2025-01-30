use std::fmt;

#[derive(Debug)]
pub enum NebulaError {
    IoError(std::io::Error),
}

impl fmt::Display for NebulaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NebulaError::IoError(err) => write!(f, "Erreur d'entrée/sortie : {}", err),
        }
    }
}

impl From<std::io::Error> for NebulaError {
    fn from(err: std::io::Error) -> Self {
        NebulaError::IoError(err)
    }
}
