use inquire::error::InquireError;
use thiserror::Error;
use toml::de::Error as TomlError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("Invalid option: {0}")]
    InvalidOptions(String),

    #[error("Invalid configuration: {0}")]
    InquireError(#[from] InquireError),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] TomlError),

    #[error("Prompt error: {0}")]
    Prompt(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("The element '{0}' already exists at the specified location.")]
    ElementAlreadyExists(String),

    #[error("The generation of this element is not implemented yet.")]
    NotImplementedError(String),
}
