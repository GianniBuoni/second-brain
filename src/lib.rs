pub mod prelude {
    pub use super::app_config::prelude::*;
    pub use super::cli::prelude::*;
    pub use super::config_file::prelude::*;
    pub use super::errors::prelude::*;
    pub(crate) use super::periodic::prelude::*;
    pub(crate) use super::{DEFAULT_DAY, DEFAULT_MONTH, DEFAULT_WEEK, DEFAULT_YEAR};
}

pub mod app;
mod app_config;
mod cli;
mod config;
mod config_file;
mod errors;
mod periodic;
mod periodic_config;

pub(crate) const DEFAULT_DAY: &str = "%Y-%m-%d";
pub(crate) const DEFAULT_WEEK: &str = "%Y-W%V";
pub(crate) const DEFAULT_MONTH: &str = "%Y-%m";
pub(crate) const DEFAULT_YEAR: &str = "%Y";
