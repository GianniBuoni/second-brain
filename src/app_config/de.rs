use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::{periodic_config::PeriodConfig, prelude::*};

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

impl TryFrom<TomlConfig> for super::AppConfig {
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

#[cfg(test)]
mod tests {
    use chrono::Local;

    use super::{super::test_configs::*, *};

    #[test]
    /// Test the required vault vaules of configuration cases
    /// Tests do not check for configuration validity, only the Toml deserialization.
    fn test_de_vault() -> anyhow::Result<()> {
        let test_cases = [
            (
                CASE_DEFAULTS,
                "./vaults",
                "Case default: Test leaving periodcal as default.",
            ),
            (
                CASE_FULL,
                "./vaults",
                "Case full: Test configuring all optional fields",
            ),
            (
                CASE_OPTIONS,
                "./vaults",
                "Case options: Test configuring a mixed bag of settings",
            ),
            (
                CASE_INVALID_VAULT,
                "invalid",
                "Case invalid: Test app configuration that should still sucessfully deserialize",
            ),
        ];
        test_cases.iter().try_for_each(|(s, want, desc)| {
            let got = toml::from_str::<TomlConfig>(s)?;

            assert_eq!(got.vault.dir, PathBuf::from(want), "{desc}");
            anyhow::Ok(())
        })
    }
    #[test]
    fn test_de_period_dir() -> anyhow::Result<()> {
        let period = Periodical::Week;
        let test_cases = [
            (
                CASE_DEFAULTS,
                None,
                "Case default: Test unconfigured parent dir",
            ),
            (
                CASE_OPTIONS,
                Some("period/week".to_string()),
                "Case options: Test configured parent dir",
            ),
        ];
        test_cases.iter().try_for_each(|(s, want, desc)| {
            let got = toml::de::from_str::<TomlConfig>(s)?;
            let got = got.periodical.unwrap_or_default().0;
            let got = got
                .get(&period)
                .unwrap_or(&PeriodConfig::default())
                .get_parent_dir()
                .map(|f| f.to_string());
            assert_eq!(*want, got, "{desc}");
            anyhow::Ok(())
        })
    }
    #[test]
    fn test_de_template_dir() -> anyhow::Result<()> {
        let period = Periodical::Year;
        let test_cases = [
            (
                CASE_OPTIONS,
                Some("templates/year.md".into()),
                "Case options: Test set template config",
            ),
            (CASE_DEFAULTS, None, "Test unset template config"),
            (
                CASE_INVALID_VAULT,
                Some("year.md".into()),
                "Case invalid: Test invalid AppConfig, but valid TomlConfig",
            ),
        ];
        test_cases.iter().try_for_each(|(s, want, desc)| {
            let got = toml::de::from_str::<TomlConfig>(s)?
                .periodical
                .unwrap_or_default()
                .0;
            let got = got
                .get(&period)
                .unwrap_or(&PeriodConfig::default())
                .get_template_file()
                .map(|f| f.to_string());
            assert_eq!(*want, got, "{desc}");
            anyhow::Ok(())
        })
    }
    #[test]
    fn test_de_filename() -> anyhow::Result<()> {
        let period = Periodical::Day;
        let test_cases = [
            (
                CASE_DEFAULTS,
                Local::now().format(DEFAULT_DAY),
                "Case default: Test unset filename formatting",
            ),
            (
                CASE_OPTIONS,
                Local::now().format("%m-%d-%Y"),
                "Case options: Test changed filename formatting",
            ),
            (
                CASE_INVALID_VAULT,
                Local::now().format(DEFAULT_DAY),
                "Case invalid: Test invalid AppConfig, but valid TomlConfig",
            ),
        ];
        test_cases.iter().try_for_each(|(s, want, desc)| {
            let got = toml::de::from_str::<TomlConfig>(s)?
                .periodical
                .unwrap_or_default()
                .0;
            let got = got
                .get(&period)
                .unwrap_or(&PeriodConfig::default())
                .format_file_name(period);
            assert_eq!(format!("{want}.md"), got, "{desc}");
            anyhow::Ok(())
        })
    }

    #[test]
    fn test_invalid_vault() -> anyhow::Result<()> {
        let desc =
            "Test invalid/non-existant vault directory in the TomlConfig to AppConfig conversion.";
        let want = ConfigError::InvalidDir("invalid".into());

        let s = CASE_INVALID_VAULT.as_bytes();
        let got = toml::de::from_slice::<TomlConfig>(s)?;
        let got = AppConfig::try_from(got);

        match got {
            Ok(e) => panic!("Expected error, got {e:?}."),
            Err(e) => assert_eq!(want.to_string(), e.to_string(), "{desc}"),
        }
        Ok(())
    }
}
