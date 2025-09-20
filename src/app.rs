use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
};

use anyhow::Result;
use chrono::Local;

use crate::prelude::*;

#[derive(Debug)]
pub struct App {
    pub config: AppConfig,
    pub command: Commands,
}

impl App {
    pub fn run(&self) -> Result<()> {
        match self.command {
            Commands::Reset => println!("Reseting config."),
            Commands::Periodical { time_span } => {
                self.check_periodical(time_span.unwrap_or_default())?;
            }
        }
        // retrun status
        Ok(())
    }
    pub fn check_periodical(&self, period: Periodical) -> Result<()> {
        let file_dir = self
            .config
            .get_periodical_dir(period)
            .unwrap_or(self.config.vault.to_owned());
        let file_name = self.get_file_name(period);
        let file_path = &file_dir.join(file_name);

        if !file_path.exists() {
            write_periodical(file_path)?;
        }

        let editor = std::env::var("EDITOR").unwrap_or("neovim".into());
        std::process::Command::new(editor).arg(file_path).status()?;

        Ok(())
    }
    // test different app configs and commands
    // test command not in configs
    fn get_file_name(&self, period: Periodical) -> String {
        let fmt = (|| -> Option<String> {
            self.config.periodical.get(&period)?.fmt.clone()
        })()
        .unwrap_or_else(|| match period {
            Periodical::Daily => DEFAULT_DAY.to_string(),
            Periodical::Weekly => DEFAULT_WEEK.to_string(),
            Periodical::Monthly => DEFAULT_MONTH.to_string(),
            Periodical::Yearly => DEFAULT_YEAR.to_string(),
        });
        let name = Local::now().format(fmt.as_str()).to_string();
        name + ".md"
    }
}

fn write_periodical(file_path: &Path) -> Result<()> {
    if let Some(prefix) = file_path.parent() {
        create_dir_all(prefix)?;
    };
    let mut f = File::create_new(file_path)?;
    f.write_all("Hello, world!".as_bytes())?;

    Ok(())
}
