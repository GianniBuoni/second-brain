use chrono::{DateTime, Days, Local, Months};
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
    /// Takes the current date and a given Periodical and formats a filename
    pub fn format_file_name(&self, period: Periodical) -> String {
        let name = Local::now().format(self.get_format(period));
        format!("{name}.md")
    }
    /// Attempts to get the configured file name associated with
    /// this Periodical.
    /// Returns a default format if not configured.
    fn get_format(&self, period: Periodical) -> &str {
        self.fmt.as_deref().unwrap_or(match period {
            Periodical::Day => DEFAULT_DAY,
            Periodical::Week => DEFAULT_WEEK,
            Periodical::Month => DEFAULT_MONTH,
            Periodical::Year => DEFAULT_YEAR,
        })
    }
    /// Given a start date and an interval of Periodcals expressed as an uint,
    /// will calculate the next interval date in time
    /// with the correct formatting.
    pub fn get_next(
        &self,
        date: DateTime<Local>,
        interval: u32,
        period: Periodical,
    ) -> Option<String> {
        let res = match period {
            Periodical::Day => date.checked_add_days(Days::new(u64::from(interval))),
            Periodical::Week => date.checked_add_days(Days::new(u64::from(interval * 7))),
            Periodical::Month => date.checked_add_months(Months::new(interval)),
            Periodical::Year => date.checked_add_months(Months::new(interval * 12)),
        };

        let fmt = self.get_format(period);
        res.map(|f| f.format(fmt).to_string())
    }
    /// Given a start date and an interval of Periodcals expressed as an uint,
    /// will calculate the next interval date in time
    /// with the correct formatting.
    pub fn get_prev(
        &self,
        date: DateTime<Local>,
        interval: u32,
        period: Periodical,
    ) -> Option<String> {
        let res = match period {
            Periodical::Day => date.checked_sub_days(Days::new(u64::from(interval))),
            Periodical::Week => date.checked_sub_days(Days::new(u64::from(interval * 7))),
            Periodical::Month => date.checked_sub_months(Months::new(interval)),
            Periodical::Year => date.checked_sub_months(Months::new(interval * 12)),
        };

        let fmt = self.get_format(period);
        res.map(|f| f.format(fmt).to_string())
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_default_filename() {
        let test_cases = [
            (Periodical::Day, DEFAULT_DAY, "test default day config"),
            (Periodical::Week, DEFAULT_WEEK, "test default week config"),
            (
                Periodical::Month,
                DEFAULT_MONTH,
                "test default month config",
            ),
            (Periodical::Year, DEFAULT_YEAR, "test default year config"),
        ];
        let config = PeriodConfig::default();

        test_cases.into_iter().for_each(|(period, fmt, desc)| {
            let want = format!("{}.md", Local::now().format(fmt));
            let got = config.format_file_name(period);
            assert_eq!(want, got, "{desc}");
        });
    }

    #[test]
    fn test_mixed_configs_filename() {
        let test_cases = [
            (Periodical::Day, "%m-%d-%Y", "test configured day fmt"),
            (Periodical::Week, DEFAULT_WEEK, "test unconfigured week fmt"),
            (Periodical::Month, "%m", "test fully configured month"),
            (Periodical::Year, DEFAULT_YEAR, "test unconfigured year"),
        ];
        let config = [
            PeriodConfig {
                fmt: Some("%m-%d-%Y".into()),
                ..Default::default()
            },
            PeriodConfig::default(),
            PeriodConfig {
                fmt: Some("%m".into()),
                ..Default::default()
            },
            PeriodConfig::default(),
        ];
        test_cases
            .into_iter()
            .zip(config)
            .for_each(|((period, fmt, desc), config)| {
                let want = format!("{}.md", Local::now().format(fmt));
                let got = config.format_file_name(period);
                assert_eq!(want, got, "{desc}");
            });
    }

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
            let got = config.get_next(date, interval, period);
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
            let got = config.get_prev(date, interval, period);
            assert_eq!(Some(want.to_string()), got)
        });
    }
}
