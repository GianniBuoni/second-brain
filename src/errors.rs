use std::path::PathBuf;

pub mod prelude {
    pub use super::{ConfigError, RuntimeError, Status};
}

#[derive(Debug, thiserror::Error)]
/// Top level error that converts, reformats, and handles any and all errors.
pub enum Status {
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("IO issue reading or writing file: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Issue with deserialization: {0}")]
    De(#[from] toml::de::Error),
    #[error("Passed in path: {0} doesn't exist or isn't a directory")]
    InvalidDir(PathBuf),
    #[error("Passed in path: {0} doesn't exist or isn't a file")]
    InvalidFile(PathBuf),
    #[error("Couldn't read file: {0}.")]
    Io(#[from] std::io::Error),
    #[error("Couldn't parse system's OS config directory.")]
    SystemDir,
}
