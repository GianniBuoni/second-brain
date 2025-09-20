use std::collections::HashMap;

use serde::Deserialize;

use super::*;

pub mod prelude {
    pub use super::AppConfig;
}

#[derive(Debug, PartialEq)]
pub struct AppConfig {
    pub vault: PathBuf,
    pub periodical: HashMap<Periodical, PeriodConfig>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TomlConfig {
    vault: TomlVault,
    periodical: TomlPeriod,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TomlVault {
    dir: PathBuf,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TomlPeriod(HashMap<Periodical, PeriodConfig>);

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PeriodConfig {
    dir: Option<String>,
    pub fmt: Option<String>,
}

impl Default for PeriodConfig {
    fn default() -> Self {
        Self {
            dir: None,
            fmt: Some(DEFAULT_DAY.into()),
        }
    }
}

impl AppConfig {
    /// Pass in a periodical to return the associated
    /// directory and file name format.
    pub fn get_periodical_dir(&self, time_span: Periodical) -> Option<PathBuf> {
        let config = self.periodical.get(&time_span)?;
        Some(self.vault.join(config.dir.as_ref()?))
    }
}

impl From<TomlConfig> for AppConfig {
    fn from(value: TomlConfig) -> Self {
        Self {
            vault: value.vault.dir,
            periodical: value.periodical.0,
        }
    }
}

impl TryFrom<&ConfigFile> for AppConfig {
    type Error = ConfigError;

    fn try_from(value: &ConfigFile) -> Result<Self, Self::Error> {
        let bytes = std::fs::read(value.path.as_path())?;
        let config = toml::from_slice::<TomlConfig>(&bytes)?;

        // validate vault dir
        let vault = config.vault.dir.as_path();
        if !vault.is_dir() {
            return Err(ConfigError::InvalidDir(vault.to_owned()));
        }

        Ok(AppConfig::from(config))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    const CASE_FULL: &str = "[vault]
dir = \"./vaults\"

[periodical.daily]
dir = \"daily\"
fmt = \"%Y-%m-%d\"";

    const CASE_OPTIONS: &str = "[vault]
dir = \"./vaults\"

[periodical.daily]
dir = \"daily\"

[periodical.yearly]
fmt = \"%Y\"";

    #[test]
    fn test_de_full() -> Result<()> {
        let period_config = PeriodConfig {
            dir: Some("day".into()),
            fmt: Some("%Y-%m-%d".into()),
        };
        let toml_period =
            TomlPeriod(HashMap::from([(Periodical::Day, period_config)]));
        let toml_vault = TomlVault {
            dir: "./vaults".into(),
        };
        let want = TomlConfig {
            vault: toml_vault,
            periodical: toml_period,
        };
        let got = toml::from_str::<TomlConfig>(CASE_FULL)?;
        assert_eq!(want, got, "Test deserialization of simple toml config");

        Ok(())
    }

    #[test]
    fn test_de_options() -> Result<()> {
        let daily_config = (
            Periodical::Day,
            PeriodConfig {
                dir: Some("day".into()),
                fmt: None,
            },
        );
        let yearly_config = (
            Periodical::Year,
            PeriodConfig {
                dir: None,
                fmt: Some("%Y".into()),
            },
        );
        let toml_period =
            TomlPeriod(HashMap::from([daily_config, yearly_config]));
        let want = TomlConfig {
            vault: TomlVault {
                dir: "./vaults".into(),
            },
            periodical: toml_period,
        };
        let got = toml::from_str::<TomlConfig>(CASE_OPTIONS)?;
        assert_eq!(
            want, got,
            "Test deserialization of toml config with None vaules."
        );

        Ok(())
    }
}
