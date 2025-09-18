use std::{env::VarError, path::PathBuf};

use thiserror::Error;

use crate::prelude::*;

pub mod prelude {
    pub use super::app_config::prelude::*;
    pub use super::config_file::prelude::*;
}

mod app_config;
mod config_file;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Couldn't parse system's OS config directory.")]
    SystemDir,
    #[error(
        "Passed in configuration path: {0} doesn't exist or isn't a directory"
    )]
    InvalidFile(PathBuf),
    #[error("Couldn't read config file: {0}.")]
    Io(#[from] std::io::Error),
    #[error("Couldn't deserialize passed in file: {0}")]
    De(#[from] toml::de::Error),
}
