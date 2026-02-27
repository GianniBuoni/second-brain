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
    pub fn open(&self, config: AppConfig) -> Result<(), Status> {
        todo!()
    }
}
