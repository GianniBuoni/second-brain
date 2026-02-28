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
        test_cases.iter().try_for_each(|(s, period, want, desc)| {
            let got =
                AppConfig::try_from(toml::de::from_str::<TomlConfig>(s)?)?.get_parent_dir(*period);
            assert_eq!(PathBuf::from(want), got, "{desc}");
            anyhow::Ok(())
        })
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
        test_cases.iter().try_for_each(|(s, period, want, desc)| {
            let got = AppConfig::try_from(toml::de::from_str::<TomlConfig>(s)?)?
                .try_format_absolute_note_path(*period)?;
            let file_name = Local::now().format(want).to_string();
            assert!(got.to_string_lossy().contains(&file_name), "{desc}");
            assert!(got.to_string_lossy().contains("vaults"), "{desc}");
            anyhow::Ok(())
        })
    }

    #[test]
    fn test_absolute_template_path() -> anyhow::Result<()> {
        let test_cases = [
            (
                CASE_DEFAULTS,
                Periodical::Week,
                None,
                "Case default: test unconfigured template file",
            ),
            (
                CASE_OPTIONS,
                Periodical::Year,
                Some("templates/year.md"),
                "Case options: test configured template file",
            ),
            (
                CASE_FULL,
                Periodical::Day,
                Some("day.md"),
                "Case full: test configured template file",
            ),
        ];
        test_cases.iter().try_for_each(|(s, period, want, desc)| {
            let got = AppConfig::try_from(toml::de::from_str::<TomlConfig>(s)?)?;
            let got = got.try_format_absolute_template_path(*period)?;
            dbg!(&got);

            match got {
                None => assert!(want.is_none(), "Expeted None: {desc}"),
                Some(path) => {
                    assert!(
                        path.to_string_lossy().contains(want.unwrap()),
                        "Check want: {desc}"
                    );
                    assert!(
                        path.to_string_lossy().contains("vaults"),
                        "Check aubsolute-ish: {desc}"
                    );
                }
            }
            anyhow::Ok(())
        })
    }
}
