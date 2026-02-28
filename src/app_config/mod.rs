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
    use chrono::Local;

    use crate::app_config::de::TomlConfig;

    use super::{test_configs::*, *};

    #[test]
    fn test_parent_dir() -> anyhow::Result<()> {
        let test_cases = [
            (
                CASE_DEFAULTS,
                Periodical::Day,
                "./vaults",
                "Test unconfigured sub-directories.",
            ),
            (
                CASE_OPTIONS,
                Periodical::Week,
                "./vaults/period/week",
                "Test configured subdirectory",
            ),
        ];
        test_cases
            .iter()
            .map(|(s, period, want, desc)| {
                let got = AppConfig::try_from(toml::de::from_str::<TomlConfig>(s)?)?
                    .get_parent_dir(*period);
                assert_eq!(PathBuf::from(want), got, "{desc}");
                anyhow::Ok(())
            })
            .collect()
    }

    #[test]
    fn test_absoulute_note_path() -> anyhow::Result<()> {
        let test_cases = [
            (
                CASE_DEFAULTS,
                Periodical::Day,
                DEFAULT_DAY,
                "Test unconfigured file names",
            ),
            (
                CASE_OPTIONS,
                Periodical::Day,
                "%m-%d-%Y",
                "Test configured file name.",
            ),
        ];
        test_cases
            .iter()
            .map(|(s, period, want, desc)| {
                let got = AppConfig::try_from(toml::de::from_str::<TomlConfig>(s)?)?
                    .format_absolute_note_path(*period)?;
                let file_name = Local::now().format(*want).to_string();
                assert!(got.to_string_lossy().contains(&file_name), "{desc}");
                assert!(got.to_string_lossy().contains("vaults"), "{desc}");
                anyhow::Ok(())
            })
            .collect()
    }
}
