use std::env;

use anyhow::Result;
use clap::Parser;

use second_brain::prelude::*;

fn main() -> Result<()> {
    let config_file =
        ConfigFile::try_from_env(env::var("SECOND_BRAIN_CONFIG"))?
            .try_build()?;
    let config = AppConfig::try_from(&config_file)?;
    let command = Args::parse().command.unwrap_or_default();

    match command {
        Commands::Reset => println!("Reseting config."),
        Commands::Periodical { time_span } => {
            second_brain::writer::check_periodical(
                &config,
                time_span.unwrap_or_default(),
            )?;
        }
    }

    Ok(())
}
