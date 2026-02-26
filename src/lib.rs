pub mod prelude {
    pub use super::cli::prelude::*;
    pub use super::config::prelude::*;
    pub(crate) use super::{DEFAULT_DAY, DEFAULT_MONTH, DEFAULT_WEEK, DEFAULT_YEAR};
}

pub mod app;
mod cli;
mod config;

pub(crate) const DEFAULT_DAY: &str = "%Y-%m-%d";
pub(crate) const DEFAULT_WEEK: &str = "%Y-W%V";
pub(crate) const DEFAULT_MONTH: &str = "%Y-%m";
pub(crate) const DEFAULT_YEAR: &str = "%Y";
