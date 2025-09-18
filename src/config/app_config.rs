use serde::Deserialize;

use super::*;

pub mod prelude {
    pub use super::AppConfig;
}

#[derive(Debug, Deserialize)]
struct TomlConfigMap {
    config: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub vault_path: PathBuf,
    daily_dir: Option<String>,
    weekly_dir: Option<String>,
    monthly_dir: Option<String>,
    yearly_dir: Option<String>,
    yearly_fmt: Option<String>,
}

impl AppConfig {
    pub fn get_periodical_dir(&self, time_span: Periodical) -> PathBuf {
        let mut root = self.vault_path.clone();
        let path = match time_span {
            Periodical::Weekly => self.weekly_dir.as_ref(),
            Periodical::Monthly => self.monthly_dir.as_ref(),
            Periodical::Yearly => self.yearly_dir.as_ref(),
            _ => self.daily_dir.as_ref(),
        };
        if let Some(p) = path {
            root = root.join(p);
        }

        root
    }
}

impl TryFrom<&ConfigFile> for AppConfig {
    type Error = ConfigError;

    fn try_from(value: &ConfigFile) -> Result<Self, Self::Error> {
        let bytes = std::fs::read(value.path.as_path())?;
        let config = toml::from_slice::<TomlConfigMap>(&bytes)?;

        // validate vault dir
        let vault = config.config.vault_path.as_path();
        if !vault.is_dir() {
            return Err(ConfigError::InvalidDir(vault.to_owned()));
        }

        Ok(config.config)
    }
}
