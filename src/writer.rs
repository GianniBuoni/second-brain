use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
};

use anyhow::Result;
use chrono::{Datelike, Local};

use crate::prelude::*;

pub fn check_periodical(
    config: &AppConfig,
    time_span: Periodical,
) -> Result<()> {
    let file_dir = config.get_periodical_dir(time_span);
    let file_name = time_span.get_file_name();
    let file_path = &file_dir.join(file_name);

    if !file_path.exists() {
        write_periodical(file_path)?;
    }

    Ok(())
}

impl Periodical {
    fn get_file_name(&self) -> String {
        let name = match self {
            Self::Weekly => format!("{:?}", Local::now().iso_week()),
            Self::Monthly => Local::now().format("%Y-%m").to_string(),
            Self::Yearly => Local::now().year().to_string(),
            _ => Local::now().date_naive().to_string(),
        };
        name + ".md"
    }
}

fn write_periodical(file_path: &Path) -> Result<()> {
    if let Some(prefix) = file_path.parent() {
        create_dir_all(prefix)?;
    };
    let mut f = File::create_new(file_path)?;
    f.write("Hello, world!".as_bytes())?;

    Ok(())
}
