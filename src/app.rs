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
                time_span.unwrap_or_default().open(&self.config)?
            }
        }
        Ok(())
    }
}
