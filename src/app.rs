use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
};

use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Runtime error. Couldn't read or parse file/dir: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct App {
    pub config: AppConfig,
    pub command: Commands,
}

impl App {
    pub fn run(&self) -> Result<(), RuntimeError> {
        match self.command {
            Commands::Reset => println!("Reseting config."),
            Commands::Periodical { time_span } => {
                self.open_periodic(time_span.unwrap_or_default())?;
            }
        }
        Ok(())
    }
    pub fn open_periodic(
        &self,
        period: Periodical,
    ) -> Result<(), RuntimeError> {
        let file_dir = self.config.get_periodical_dir(period)?;
        let file_name = self.config.get_file_name(period);
        let file_path = &file_dir.join(file_name);
        // write a new files if target does not exist
        if !file_path.exists() {
            write_periodical(file_path)?;
        }
        // open file in editor
        let editor = std::env::var("EDITOR").unwrap_or("nvim".into());
        std::env::set_current_dir(self.config.get_vault_root())?;
        std::process::Command::new(editor).arg(file_path).status()?;

        Ok(())
    }
}

fn write_periodical(file_path: &Path) -> Result<(), RuntimeError> {
    if let Some(prefix) = file_path.parent() {
        create_dir_all(prefix)?;
    };
    let mut f = File::create_new(file_path)?;
    f.write_all("Hello, world!".as_bytes())?;

    Ok(())
}
