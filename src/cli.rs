use clap::{Parser, Subcommand};
use strum::VariantNames;
use strum_macros::{Display, EnumString, VariantNames};

pub mod prelude {
    pub use super::{Args, Commands, Periodical};
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Display, Clone, Subcommand)]
pub enum Commands {
    ///  Opens up passed in periodical note
    #[clap(short_flag = 'p',long_about=periodical_help())]
    Periodical {
        #[arg(help = format!("{:?}", Periodical::VARIANTS))]
        time_span: Option<Periodical>,
    },
    /// Resets the app configuration to its default state
    #[clap(short_flag = 'r')]
    Reset,
}

impl Default for Commands {
    fn default() -> Self {
        Self::Periodical {
            time_span: Some(Periodical::default()),
        }
    }
}

#[derive(Debug, Default, Display, Clone, Copy, EnumString, VariantNames, Parser)]
#[strum(serialize_all = "kebab-case")]
pub enum Periodical {
    #[default]
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

fn periodical_help() -> String {
    format!(
        "Opens up passed in periodical note\n\nThis command will open your $EDITOR for your corresponding note. If none exists, then one will be written.\nArgument options are {:?}.\nsecond-brain will default to passing in \"daily\" if no argument is given.",
        Periodical::VARIANTS
    )
}
