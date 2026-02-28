use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{periodic_config::PeriodConfig, prelude::*};

pub mod prelude {
    pub use super::AppConfig;
}

mod de;
#[cfg(test)]
mod test_configs;

#[derive(Debug, PartialEq)]
pub struct AppConfig {
    vault: PathBuf,
    periodical: HashMap<Periodical, PeriodConfig>,
}

impl AppConfig {
    fn get_vault_root(&self) -> &Path {
        &self.vault
    }
    /// Attempts to get the parent directory of a note file.
    /// Returns the valut root if none is configured.
    fn get_parent_dir(&self, period: Periodical) -> PathBuf {
        let dir = || {
            let config = self.periodical.get(&period)?;
            Some(config.get_parent_dir()?)
        };
        match dir() {
            Some(p) => self.get_vault_root().join(p),
            None => self.vault.clone(),
        }
    }
    /// Attempts to format and return the absolute path of a note file.
    /// Returns `${VAULT_ROOT}/${DEFAULT_FILE_NAME_FORMAT}.md` if
    /// nothing was configured.
    fn format_absolute_note_path(&self, period: Periodical) -> Result<PathBuf, RuntimeError> {
        let parent_dir = self.get_parent_dir(period);
        let file_name = self
            .periodical
            .get(&period)
            .unwrap_or(&PeriodConfig::default())
            .format_file_name(period);
        let mut full_path = PathBuf::from(parent_dir).join(file_name);

        if full_path.is_relative() {
            full_path = std::path::absolute(full_path)?;
        }
        Ok(full_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the default config creates test cases where the failover
    /// configurations will be used.
    fn default_config() -> AppConfig {
        AppConfig {
            vault: "./vault".into(),
            periodical: HashMap::new(),
        }
    }

    #[test]
    fn test_parent_dir() {
        let test_cases = [(
            Periodical::Day,
            default_config(),
            "Test unconfigured sub-directories.",
        )];
        todo!()
    }

    #[test]
    fn test_absoulute_note_path() {
        let test_cases = [(
            Periodical::Day,
            default_config(),
            "Test unconfigured file names",
        )];
        todo!()
    }
}
