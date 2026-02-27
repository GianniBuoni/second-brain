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
mod math;

#[derive(Debug, PartialEq)]
pub struct AppConfig {
    pub vault: PathBuf,
    pub periodical: HashMap<Periodical, PeriodConfig>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PeriodConfig {
    pub dir: Option<String>,
    template: Option<PathBuf>,
    pub fmt: Option<String>,
}

impl AppConfig {
    /// returns the configured root directory of the vault
    pub fn get_vault_root(&self) -> PathBuf {
        self.vault.clone()
    }
    /// pass in a periodical to return the associated
    /// directory and file name format.
    pub fn get_periodical_dir(&self, period: Periodical) -> Result<PathBuf, std::io::Error> {
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
        let fmt = (|| -> Option<String> { self.periodical.get(&period)?.fmt.clone() })()
            .unwrap_or_else(|| match period {
                Periodical::Day => DEFAULT_DAY.to_string(),
                Periodical::Week => DEFAULT_WEEK.to_string(),
                Periodical::Month => DEFAULT_MONTH.to_string(),
                Periodical::Year => DEFAULT_YEAR.to_string(),
            });
        let name = Local::now().format(fmt.as_str()).to_string();

        name + ".md"
    }
    /// pass in a periodical to get the path to its template
    pub fn get_template_path(&self, period: Periodical) -> Option<PathBuf> {
        let period_config = self.periodical.get(&period)?;
        let template_path = self.vault.join(period_config.template.as_ref()?);

        Some(template_path)
    }
    /// pass in a path, validate that it is a file, and get its contents
    pub fn get_template_contents(&self, path: PathBuf) -> Result<Vec<u8>, ConfigError> {
        let mut path = path;
        if !path.is_file() {
            return Err(ConfigError::InvalidFile(path));
        }
        if path.is_relative() {
            path = std::path::absolute(path)?;
        }

        Ok(std::fs::read(path)?)
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
                    template: None,
                },
            ),
            (
                Periodical::Week,
                PeriodConfig {
                    dir: Some("period/week".into()),
                    fmt: None,
                    template: Some("week.md".into()),
                },
            ),
            (
                Periodical::Month,
                PeriodConfig {
                    dir: Some("month".into()),
                    fmt: Some("%m".into()),
                    template: Some("month.md".into()),
                },
            ),
        ]);
        AppConfig {
            vault: "./vault".into(),
            periodical,
        }
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

    #[test]
    fn test_get_template_path() {
        let test_cases = [
            (Periodical::Day, None),
            (Periodical::Week, Some("./vault/week.md")),
            (Periodical::Month, Some("./vault/month.md")),
            (Periodical::Year, None),
        ];
        let app_config = mixed_config();

        test_cases.into_iter().for_each(|(period, want)| {
            let want = want.map(PathBuf::from);
            let got = app_config.get_template_path(period);
            assert_eq!(want, got);
        });
    }
}
