use std::{env, path::PathBuf};

use crate::prelude::*;

pub mod prelude {
    pub use super::ConfigFile;
}

/// Struct that keeps track of the state of configuration I/O.
/// This is newtype for the path buffer to the configuration file
/// and ensures that the enclosed PathBuf has been validated.
pub struct ConfigFile(pub PathBuf);

/// Type state used to prevent AppConfig from consuming
/// a non-inialized/invalid config file
pub struct ConfigFileBuilder(PathBuf);

impl ConfigFile {
    /// Attempts to initialize a ConfigFileBuilder.
    /// If the env variable is not set, the program will provide
    /// a default file path.
    pub fn try_from_env(s: &str) -> Result<ConfigFileBuilder, ConfigError> {
        let path = match env::var(s) {
            Ok(s) => PathBuf::from(s),
            Err(_) => {
                let root = dirs::config_dir().ok_or(ConfigError::SystemDir)?;
                root.join("sb_config.toml")
            }
        };
        Ok(ConfigFileBuilder(path))
    }
}

impl ConfigFileBuilder {
    /// Type conversion that implicitly validates the ConfigFileBuilder's
    /// interior PathBuf
    pub fn try_build(self) -> Result<ConfigFile, ConfigError> {
        if !self.0.is_file() {
            return Err(ConfigError::InvalidFile(self.0));
        }
        Ok(ConfigFile(self.0))
    }
}
