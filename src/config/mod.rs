use std::{collections::HashMap, env::VarError, path::PathBuf};

use chrono::Local;
use serde::Deserialize;
use thiserror::Error;

use crate::prelude::*;

pub mod prelude {
    pub use super::AppConfig;
}

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
