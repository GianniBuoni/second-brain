use super::*;

pub mod prelude {
    pub use super::ConfigFile;
}

/// struct that keeps track of the state of configuration I/O.
pub struct ConfigFile {
    pub(crate) path: PathBuf,
}

/// type state used to prevent AppConfig from consuming
/// a non-inialized/valid config file
pub struct ConfigFileBuilder {
    path: PathBuf,
}

impl ConfigFile {
    /// pass in a path from an enviroment variable
    pub fn try_from_env(
        s: std::result::Result<String, VarError>,
    ) -> Result<ConfigFileBuilder, ConfigError> {
        let path = match s {
            Ok(s) => PathBuf::from(s),
            Err(_) => {
                let root = dirs::config_dir().ok_or(ConfigError::SystemDir)?;
                root.join("sb_config.toml")
            }
        };
        Ok(ConfigFileBuilder { path })
    }
}

impl ConfigFileBuilder {
    pub fn try_build(self) -> Result<ConfigFile, ConfigError> {
        if !self.path.is_file() {
            return Err(ConfigError::InvalidFile(self.path));
        }
        Ok(ConfigFile { path: self.path })
    }
}
