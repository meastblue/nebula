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

    #[error("Not a Nebula project directory")]
    NotNebulaProject,

    #[error("Invalid project structure")]
    InvalidProjectStructure,

    #[error("Invalid relation format: {0}")]
    InvalidRelationFormat(String),

    #[error("Invalid relation type: {0}")]
    InvalidRelationType(String),

    #[error("Invalid field format: {0}. Expected format: name:type|validation1 validation2")]
    InvalidFieldFormat(String),

    #[error("Missing type for field: {0}")]
    MissingTypForField(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid path: Path is not valid or cannot be processed")]
    InvalidPath,

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
}
