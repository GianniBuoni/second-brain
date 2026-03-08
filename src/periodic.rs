use std::{io::Write, path::Path};

use chrono::{DateTime, Days, Local, Months};
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
    pub fn open(&self, config: &AppConfig) -> Result<(), Status> {
        let date = Local::now();
        let path = config.try_format_absolute_note_path(*self, date)?;
        // write file if it doesn't exist
        if !path.exists() {
            self.write(config, &path, date)?;
        }
        // open file in editor
        let editor = std::env::var("EDITOR").unwrap_or("nvim".into());
        std::env::set_current_dir(config.get_vault_root()).map_err(RuntimeError::Io)?;
        std::process::Command::new(editor)
            .arg(path)
            .status()
            .map_err(RuntimeError::Io)?;

        Ok(())
    }
    fn write(&self, config: &AppConfig, path: &Path, date: DateTime<Local>) -> Result<(), Status> {
        let mut contents = Vec::<u8>::new();
        if let Some(heading) = self.try_format_heading(config, date) {
            contents.append(&mut heading.into());
            contents.append(&mut "\n\n".to_string().into());
        }
        if let Some(template_path) = config.try_format_absolute_template_path(*self)? {
            // validate template before reading
            if !template_path.is_file() {
                return Err(ConfigError::InvalidFile(template_path).into());
            }
            let mut template = std::fs::read(template_path).map_err(RuntimeError::Io)?;
            contents.append(&mut template);
        }
        // create any necessary parent dirs
        if let Some(parent_path) = path.parent() {
            std::fs::create_dir_all(parent_path).map_err(RuntimeError::Io)?;
        }
        // create file
        let mut f = std::fs::File::create_new(path).map_err(RuntimeError::Io)?;
        // write any template contents
        if !contents.is_empty() {
            f.write(&contents).map_err(RuntimeError::Io)?;
        }
        Ok(())
    }
    fn try_format_heading(&self, config: &AppConfig, date: DateTime<Local>) -> Option<String> {
        let prev = config.format_date(*self, self.get_prev(date, 1)?);
        let next = config.format_date(*self, self.get_next(date, 1)?);

        Some(format!("[[{prev}]] - [[{next}]]"))
    }
    /// Given a start date and an interval of Periodcals expressed as an uint,
    /// will calculate the next interval date in time
    /// with the correct formatting.
    fn get_next(&self, date: DateTime<Local>, interval: u32) -> Option<DateTime<Local>> {
        match self {
            Periodical::Day => date.checked_add_days(Days::new(u64::from(interval))),
            Periodical::Week => date.checked_add_days(Days::new(u64::from(interval * 7))),
            Periodical::Month => date.checked_add_months(Months::new(interval)),
            Periodical::Year => date.checked_add_months(Months::new(interval * 12)),
        }
    }
    /// Given a start date and an interval of Periodcals expressed as an uint,
    /// will calculate the next interval date in time
    /// with the correct formatting.
    fn get_prev(&self, date: DateTime<Local>, interval: u32) -> Option<DateTime<Local>> {
        match self {
            Periodical::Day => date.checked_sub_days(Days::new(u64::from(interval))),
            Periodical::Week => date.checked_sub_days(Days::new(u64::from(interval * 7))),
            Periodical::Month => date.checked_sub_months(Months::new(interval)),
            Periodical::Year => date.checked_sub_months(Months::new(interval * 12)),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::super::periodic_config::PeriodConfig;
    use super::*;

    #[test]
    fn test_get_next() {
        let date = Local.with_ymd_and_hms(2025, 12, 30, 0, 0, 0).unwrap();
        let config = PeriodConfig::default();

        let test_cases = [
            (Periodical::Day, 1, "2025-12-31"),
            (Periodical::Week, 1, "2026-W02"),
            (Periodical::Month, 1, "2026-01"),
            (Periodical::Year, 2, "2027"),
        ];

        test_cases.into_iter().for_each(|(period, interval, want)| {
            let got = period
                .get_next(date, interval)
                .map(|f| config.format(period, f));

            assert_eq!(Some(want.to_string()), got)
        });
    }

    #[test]
    fn test_get_prev() {
        let date = Local.with_ymd_and_hms(2025, 12, 30, 0, 0, 0).unwrap();
        let config = PeriodConfig::default();

        let test_cases = [
            (Periodical::Day, 1, "2025-12-29"),
            (Periodical::Week, 1, "2025-W52"),
            (Periodical::Month, 10, "2025-02"),
            (Periodical::Year, 2, "2023"),
        ];

        test_cases.into_iter().for_each(|(period, interval, want)| {
            let got = period
                .get_prev(date, interval)
                .map(|f| config.format(period, f));

            assert_eq!(Some(want.to_string()), got)
        });
    }
    #[test]
    fn test_headings() {
        let desc = "Test template heading genereation";
        let date = Local.with_ymd_and_hms(2025, 12, 30, 0, 0, 0).unwrap();
        let config = AppConfig::default();

        let test_cases = [
            (Periodical::Day, "[[2025-12-29]] - [[2025-12-31]]"),
            (Periodical::Week, "[[2025-W52]] - [[2026-W02]]"),
            (Periodical::Month, "[[2025-11]] - [[2026-01]]"),
            (Periodical::Year, "[[2024]] - [[2026]]"),
        ];

        test_cases.into_iter().for_each(|(period, want)| {
            let got = period.try_format_heading(&config, date);
            assert_eq!(Some(want.into()), got, "{desc}: {period}")
        });
    }
}
