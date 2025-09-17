use anyhow::Result;
use clap::Parser;

use second_brain::prelude::*;

fn main() -> Result<()> {
    // make config
    let config_file = get_config_dir()?;
    let config = build_config(&config_file)?;

    let command = Args::parse().command.unwrap_or_default();

    match command {
        Commands::Reset => println!("Reseting confg."),
        Commands::Periodical { time_span } => {
            print_preiodical(&config, time_span.unwrap_or_default());
        }
    }

    Ok(())
}

fn print_preiodical(config: &AppConfig, time_span: Periodical) {
    let dir = config.get_periodical_dir(time_span);

    println!("Opening {time_span} note at {dir:?}")
}
