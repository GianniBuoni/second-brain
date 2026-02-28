use std::path::Path;

use crate::prelude::*;

#[derive(Debug)]
pub struct App {
    pub config: AppConfig,
    pub command: Commands,
}

impl App {
    pub fn run(&self) -> Result<(), Status> {
        match self.command {
            Commands::Reset => println!("Reseting config."),
            Commands::Periodical { time_span } => {
                self.open_periodic(time_span.unwrap_or_default())?;
            }
        }
        Ok(())
    }
    pub fn open_periodic(&self, period: Periodical) -> Result<(), RuntimeError> {
        todo!()
    }
    fn write_periodical(&self, file_path: &Path, period: Periodical) -> anyhow::Result<()> {
        todo!()
    }
}
