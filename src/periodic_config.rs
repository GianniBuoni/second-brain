use chrono::{DateTime, Datelike, Local};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct PeriodConfig {
    dir: Option<String>,
    template: Option<String>,
    fmt: Option<String>,
}

impl PeriodConfig {
    /// Getter that returns an Optional for the configured parent directory
    /// for the periodical note file.
    pub fn get_parent_dir(&self) -> Option<&str> {
        Some(self.dir.as_ref()?.as_str())
    }
    /// Getter that returns an Optional for the configured template file
    ///  location for the periodical note file.
    pub fn get_template_file(&self) -> Option<&str> {
        Some(self.template.as_ref()?.as_str())
    }
    /// Attempts to get the configured file name associated with
    /// this Periodical.
    /// Returns a default format if not configured.
    pub fn format(&self, period: Periodical, date: DateTime<Local>) -> String {
        let fmt = self.fmt.as_deref().unwrap_or(match period {
            Periodical::Day => DEFAULT_DAY,
            Periodical::Week => DEFAULT_WEEK,
            Periodical::Month => DEFAULT_MONTH,
            Periodical::Year => DEFAULT_YEAR,
        });
        if matches!(period, Periodical::Week) {
            let year = date.iso_week().year();
            let year = match year.abs() < 10 {
                true => format!("0{year}"),
                false => year.to_string(),
            };
            let week = date.iso_week().week();
            let week = match week < 10 {
                true => format!("0{week}"),
                false => week.to_string(),
            };
            let res = fmt.replace("%Y", &year);
            let res = res.replace("%V", &week);
            return res;
        }
        date.format(fmt).to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_default_formatter() {
        let desc = "Test default configs";
        let date = Local.with_ymd_and_hms(2025, 12, 30, 0, 0, 0).unwrap();
        let config = PeriodConfig::default();

        let test_cases = [
            (Periodical::Day, "2025-12-30"),
            (Periodical::Week, "2026-W01"),
            (Periodical::Month, "2025-12"),
            (Periodical::Year, "2025"),
        ];

        test_cases.into_iter().for_each(|(period, want)| {
            let got = config.format(period, date);
            assert_eq!(want, got, "{desc}: {period}");
        });
    }

    #[test]
    fn test_mixed_configs_filename() {
        let desc = "Test a mix of configured and unconfigrued date formats";
        let date = Local.with_ymd_and_hms(2025, 12, 30, 0, 0, 0).unwrap();
        let test_cases = [
            (Periodical::Day, "12-30-2025", "configured day"),
            (Periodical::Week, "01-2026", "configured week"),
            (Periodical::Month, "12", "configured month"),
            (Periodical::Year, "2025", "test unconfigured year"),
        ];
        let config = [
            PeriodConfig {
                fmt: Some("%m-%d-%Y".into()),
                ..Default::default()
            },
            PeriodConfig {
                fmt: Some("%V-%Y".into()),
                ..Default::default()
            },
            PeriodConfig {
                fmt: Some("%m".into()),
                ..Default::default()
            },
            PeriodConfig::default(),
        ];
        test_cases
            .into_iter()
            .zip(config)
            .for_each(|((period, want, case), config)| {
                let got = config.format(period, date);
                assert_eq!(want, got, "{desc}: {case} {period}");
            });
    }
}
