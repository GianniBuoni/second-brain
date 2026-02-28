use std::{io::Write, path::Path};

use clap::Parser;
use serde::Deserialize;
use strum_macros::{Display, EnumString, VariantNames};

use crate::prelude::*;

pub mod prelude {
    pub use super::Periodical;
}

#[derive(
    Debug,
    Default,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Deserialize,
    EnumString,
    Parser,
    VariantNames,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Periodical {
    #[default]
    Day,
    Week,
    Month,
    Year,
}

impl Periodical {
    pub fn open(&self, config: &AppConfig) -> Result<(), Status> {
        let path = config.try_format_absolute_note_path(*self)?;
        // write file if it doesn't exist
        if !path.exists() {
            self.write(config, &path)?;
        }
        // open file in editor
        let editor = std::env::var("EDITOR").unwrap_or("nvim".into());
        std::env::set_current_dir(config.get_vault_root()).map_err(RuntimeError::Io)?;
        std::process::Command::new(editor)
            .arg(path)
            .status()
            .map_err(RuntimeError::Io)?;

        Ok(())
    }
    fn write(&self, config: &AppConfig, path: &Path) -> Result<(), Status> {
        let mut contents = Vec::<u8>::new();
        if let Some(template_path) = config.try_format_absolute_template_path(*self)? {
            // validate template before reading
            if !template_path.is_file() {
                return Err(ConfigError::InvalidFile(template_path).into());
            }
            contents = std::fs::read(template_path).map_err(RuntimeError::Io)?;
        }
        // create any necessary parent dirs
        if let Some(parent_path) = path.parent() {
            std::fs::create_dir_all(parent_path).map_err(RuntimeError::Io)?;
        }
        // create file
        let mut f = std::fs::File::create_new(path).map_err(RuntimeError::Io)?;
        // write any template contents
        if !contents.is_empty() {
            f.write(&contents).map_err(RuntimeError::Io)?;
        }
        Ok(())
    }
}
