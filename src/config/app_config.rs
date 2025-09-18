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
    pub vault_path: String,
    daily_dir: Option<String>,
    weekly_dir: Option<String>,
    monthly_dir: Option<String>,
    yearly_dir: Option<String>,
    yearly_fmt: Option<String>,
}

impl AppConfig {
    pub fn get_periodical_dir(&self, time_span: Periodical) -> PathBuf {
        let path = match time_span {
            Periodical::Weekly => self.weekly_dir.as_ref(),
            Periodical::Monthly => self.monthly_dir.as_ref(),
            Periodical::Yearly => self.yearly_dir.as_ref(),
            _ => self.daily_dir.as_ref(),
        };
        let Some(path) = path else {
            return PathBuf::from(self.vault_path.as_str());
        };
        PathBuf::from(self.vault_path.as_str()).join(path)
    }
}

impl TryFrom<&ConfigFile> for AppConfig {
    type Error = ConfigError;

    fn try_from(value: &ConfigFile) -> Result<Self, Self::Error> {
        let bytes = std::fs::read(value.path.as_path())?;
        let config = toml::from_slice::<TomlConfigMap>(&bytes)?;

        Ok(config.config)
    }
}
