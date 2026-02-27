use clap::Parser;

use second_brain::prelude::*;

fn main() -> Result<(), Status> {
    let config_file = ConfigFile::try_from_env("SECOND_BRAIN_CONFIG")?.try_build()?;
    let config = AppConfig::try_from(config_file)?;
    let command = Args::parse().command.unwrap_or_default();

    let app = second_brain::app::App { config, command };
    app.run()?;

    Ok(())
}
