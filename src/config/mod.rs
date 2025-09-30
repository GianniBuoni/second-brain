use std::{collections::HashMap, env::VarError, path::PathBuf};

use chrono::Local;
use serde::Deserialize;
use thiserror::Error;

use crate::prelude::*;

pub mod prelude {
    pub use super::AppConfig;
    pub use super::config_file::prelude::*;
}

mod config_file;
mod de;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config error. Couldn't parse system's OS config directory.")]
    SystemDir,
    #[error("Config error. Passed in path: {0} doesn't exist or isn't a file")]
    InvalidFile(PathBuf),
    #[error(
        "Config error. Passed in path: {0} doesn't exist or isn't a directory"
    )]
    InvalidDir(PathBuf),
    #[error("Config error. Couldn't read config file: {0}.")]
    Io(#[from] std::io::Error),
    #[error("Config error. Couldn't deserialize passed in file: {0}")]
    De(#[from] toml::de::Error),
}

#[derive(Debug, PartialEq)]
pub struct AppConfig {
    vault: PathBuf,
    pub periodical: HashMap<Periodical, PeriodConfig>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PeriodConfig {
    dir: Option<String>,
    pub fmt: Option<String>,
}

impl AppConfig {
    /// returns the configured root directory of the vault
    pub fn get_vault_root(&self) -> PathBuf {
        self.vault.clone()
    }
    /// Pass in a periodical to return the associated
    /// directory and file name format.
    pub fn get_periodical_dir(
        &self,
        period: Periodical,
    ) -> Result<PathBuf, std::io::Error> {
        let mut dir = (|| -> Option<PathBuf> {
            let config = self.periodical.get(&period)?;
            Some(self.vault.join(config.dir.as_ref()?))
        })()
        .unwrap_or(self.vault.to_path_buf());
        // convert any relative path into an absolute one
        if dir.is_relative() {
            dir = std::path::absolute(dir)?;
        }

        Ok(dir)
    }
    pub fn get_file_name(&self, period: Periodical) -> String {
        let fmt = (|| -> Option<String> {
            self.periodical.get(&period)?.fmt.clone()
        })()
        .unwrap_or_else(|| match period {
            Periodical::Day => DEFAULT_DAY.to_string(),
            Periodical::Week => DEFAULT_WEEK.to_string(),
            Periodical::Month => DEFAULT_MONTH.to_string(),
            Periodical::Year => DEFAULT_YEAR.to_string(),
        });
        let name = Local::now().format(fmt.as_str()).to_string();
        name + ".md"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    fn default_config() -> AppConfig {
        AppConfig {
            vault: "./vault".into(),
            periodical: HashMap::new(),
        }
    }

    fn mixed_config() -> AppConfig {
        let periodical = HashMap::from([
            (
                Periodical::Day,
                PeriodConfig {
                    dir: None,
                    fmt: Some("%m-%d-%Y".into()),
                },
            ),
            (
                Periodical::Week,
                PeriodConfig {
                    dir: Some("period/week".into()),
                    fmt: None,
                },
            ),
            (
                Periodical::Month,
                PeriodConfig {
                    dir: Some("month".into()),
                    fmt: Some("%m".into()),
                },
            ),
        ]);
        AppConfig {
            vault: "./vault".into(),
            periodical,
        }
    }

    #[test]
    fn test_default_filename() {
        let test_cases = [
            (Periodical::Day, DEFAULT_DAY, "test default day config"),
            (Periodical::Week, DEFAULT_WEEK, "test default week config"),
            (
                Periodical::Month,
                DEFAULT_MONTH,
                "test default month config",
            ),
            (Periodical::Year, DEFAULT_YEAR, "test default year config"),
        ];
        let app_config = default_config();

        test_cases.into_iter().for_each(|(period, fmt, desc)| {
            let want = format!("{}.md", Local::now().format(fmt));
            let got = app_config.get_file_name(period);
            assert_eq!(want, got, "{desc}");
        });
    }

    #[test]
    fn test_mixed_configs_filename() {
        let test_cases = [
            (Periodical::Day, "%m-%d-%Y", "test configured day fmt"),
            (Periodical::Week, DEFAULT_WEEK, "test unconfigured week fmt"),
            (Periodical::Month, "%m", "test fully configured month"),
            (Periodical::Year, DEFAULT_YEAR, "test unconfigured year"),
        ];
        let app_config = mixed_config();

        test_cases.into_iter().for_each(|(period, fmt, desc)| {
            let want = format!("{}.md", Local::now().format(fmt));
            let got = app_config.get_file_name(period);
            assert_eq!(want, got, "{desc}");
        });
    }

    #[test]
    fn test_default_periodical_dir() -> Result<()> {
        let test_cases = [
            Periodical::Day,
            Periodical::Week,
            Periodical::Month,
            Periodical::Year,
        ];
        let app_config = default_config();
        test_cases.into_iter().try_for_each(|f| {
            let want = "/vault";
            let got = app_config.get_periodical_dir(f)?;
            assert!(
                got.to_string_lossy().contains(want),
                "test default {} config",
                f
            );
            anyhow::Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn test_configured_periodical_dir() -> Result<()> {
        let test_cases = [
            (Periodical::Day, "/vault", "test None dayr dir"),
            (
                Periodical::Week,
                "/vault/period/week",
                "test configured week dir",
            ),
            (
                Periodical::Month,
                "/vault/month",
                "test configured month dir",
            ),
            (Periodical::Year, "/vault", "test unconfigured year"),
        ];
        let app_config = mixed_config();

        test_cases
            .into_iter()
            .try_for_each(|(period, want, desc)| {
                let got = app_config.get_periodical_dir(period)?;
                assert!(got.to_string_lossy().contains(want), "{desc}");

                anyhow::Ok(())
            })?;

        Ok(())
    }

    #[test]
    fn test_vault_root() {
        let test_cases = ["day", "month", "week", "year"];
        let app_config = mixed_config();

        test_cases.into_iter().for_each(|exclude| {
            let got = app_config.get_vault_root();
            let got = got.to_string_lossy();

            assert!(
                !got.contains(exclude),
                "test if get_vault_root() doesn't use periodic folders"
            )
        });
    }
}
