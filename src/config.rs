use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;

use crate::prelude::*;

pub mod prelude {
    pub use super::{AppConfig, build_config, get_config_dir};
}

#[derive(Debug, Deserialize)]
struct TomlConfigMap {
    config: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub vault_path: String,
    daily_dir: Option<String>,
    weekly_dir: Option<String>,
    mothly_dir: Option<String>,
    yearly_dir: Option<String>,
    yearly_fmt: Option<String>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Couldn't parse system's OS config directory.")]
    SystemDir,
    #[error("Couldn't read config file: {0}.")]
    Io(#[from] std::io::Error),
    #[error("Couldn't deserialize passed in file: {0}")]
    De(#[from] toml::de::Error),
}

impl AppConfig {
    pub fn get_periodical_dir(&self, time_span: Periodical) -> PathBuf {
        let path = match time_span {
            Periodical::Weekly => self.weekly_dir.as_ref(),
            Periodical::Monthly => self.mothly_dir.as_ref(),
            Periodical::Yearly => self.yearly_dir.as_ref(),
            _ => self.daily_dir.as_ref(),
        };
        let Some(path) = path else {
            return PathBuf::from(self.vault_path.as_str());
        };
        PathBuf::from(self.vault_path.as_str()).join(path)
    }
}

/// Looks for a $SECOND_BRAIN_CONFIG environment variable
/// and returns a configuration PathBuff depending on whether
/// the environment varaible has been set.
pub fn get_config_dir() -> Result<PathBuf, ConfigError> {
    if let Ok(s) = std::env::var("SECOND_BRAIN_CONFIG") {
        return Ok(PathBuf::from(s));
    };
    let root = dirs::config_dir().ok_or(ConfigError::SystemDir)?;
    Ok(root.join("sb_config.toml"))
}

pub fn build_config(s: &std::path::Path) -> Result<AppConfig, ConfigError> {
    let bytes = std::fs::read(s)?;
    let config = toml::from_slice::<TomlConfigMap>(&bytes)?;

    Ok(config.config)
}
