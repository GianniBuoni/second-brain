use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{periodic_config::PeriodConfig, prelude::*};

pub mod prelude {
    pub use super::AppConfig;
}

#[cfg(test)]
mod test_cases;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct AppConfig {
    vault: PathBuf,
    periodical: HashMap<Periodical, PeriodConfig>,
}

impl AppConfig {
    /// Attempts to get the parent directory of a note file.
    /// Returns the valut root if none is configured.
    fn get_parent_dir(&self, period: Periodical) -> PathBuf {
        let dir = || {
            let config = self.periodical.get(&period)?;
            config.get_parent_dir()
        };
        match dir() {
            Some(p) => self.get_vault_root().join(p),
            None => self.vault.clone(),
        }
    }
    pub fn get_vault_root(&self) -> &Path {
        &self.vault
    }
    /// Attempts to format and return the absolute path of a note file.
    /// Returns `${VAULT_ROOT}/${DEFAULT_FILE_NAME_FORMAT}.md` if
    /// nothing was configured.
    pub fn try_format_absolute_note_path(
        &self,
        period: Periodical,
    ) -> Result<PathBuf, RuntimeError> {
        let parent_dir = self.get_parent_dir(period);
        let file_name = self
            .periodical
            .get(&period)
            .unwrap_or(&PeriodConfig::default())
            .format_file_name(period);
        let mut full_path = parent_dir.join(file_name);

        if full_path.is_relative() {
            full_path = std::path::absolute(full_path)?;
        }
        Ok(full_path)
    }
    /// Attempts to get the absolute path of a possible template file.
    pub fn try_format_absolute_template_path(
        &self,
        period: Periodical,
    ) -> Result<Option<PathBuf>, RuntimeError> {
        let path = || self.periodical.get(&period)?.get_template_file();
        if let Some(path) = path() {
            let mut path = self.get_vault_root().join(path);
            if !path.is_absolute() {
                path = std::path::absolute(path)?;
            }
            dbg!(&path);
            return Ok(Some(path));
        }
        Ok(None)
    }
}

/// Raw TOML deserialization of the AppConfig
#[derive(Debug, Deserialize, PartialEq)]
pub struct TomlConfig {
    vault: TomlVault,
    periodical: Option<TomlPeriod>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TomlVault {
    dir: PathBuf,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
struct TomlPeriod(HashMap<Periodical, PeriodConfig>);

impl TryFrom<TomlConfig> for AppConfig {
    type Error = ConfigError;

    fn try_from(value: TomlConfig) -> Result<Self, Self::Error> {
        // validate vauld diretory
        let vault = value.vault.dir.as_path();
        if !vault.is_dir() {
            return Err(ConfigError::InvalidDir(vault.to_owned()));
        }
        Ok(Self {
            vault: value.vault.dir,
            periodical: value.periodical.unwrap_or_default().0,
        })
    }
}

impl TryFrom<ConfigFile> for TomlConfig {
    type Error = ConfigError;

    fn try_from(value: ConfigFile) -> Result<Self, Self::Error> {
        let bytes = std::fs::read(value.0)?;
        Ok(toml::from_slice::<TomlConfig>(&bytes)?)
    }
}

impl TryFrom<ConfigFile> for AppConfig {
    type Error = ConfigError;

    fn try_from(value: ConfigFile) -> Result<Self, Self::Error> {
        let toml = TomlConfig::try_from(value)?;
        AppConfig::try_from(toml)
    }
}
