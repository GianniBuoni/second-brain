use super::*;

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

[periodical.day]
dir = \"day\"
fmt = \"%Y-%m-%d\"
template = \"day.md\"";

    const CASE_OPTIONS: &str = "[vault]
dir = \"./vaults\"

[periodical.day]
dir = \"day\"

[periodical.year]
fmt = \"%Y\"
template = \"year.md\"";

    #[test]
    fn test_de_full() -> Result<()> {
        let period_config = PeriodConfig {
            dir: Some("day".into()),
            fmt: Some("%Y-%m-%d".into()),
            template: Some("day.md".into()),
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
                template: None,
            },
        );
        let yearly_config = (
            Periodical::Year,
            PeriodConfig {
                dir: None,
                fmt: Some("%Y".into()),
                template: Some("year.md".into()),
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
