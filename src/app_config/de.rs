use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::{periodic_config::PeriodConfig, prelude::*};

/// Raw TOML deserialization of the AppConfig
#[derive(Debug, Deserialize, PartialEq)]
pub struct TomlConfig {
    vault: TomlVault,
    periodical: TomlPeriod,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TomlVault {
    dir: PathBuf,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TomlPeriod(HashMap<Periodical, PeriodConfig>);

impl From<TomlConfig> for super::AppConfig {
    fn from(value: TomlConfig) -> Self {
        Self {
            vault: value.vault.dir,
            periodical: value.periodical.0,
        }
    }
}
