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
