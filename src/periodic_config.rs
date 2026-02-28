use chrono::Local;
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
    pub fn format_file_name(&self, period: Periodical) -> String {
        let fmt = self
            .fmt
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_else(|| match period {
                Periodical::Day => DEFAULT_DAY,
                Periodical::Week => DEFAULT_WEEK,
                Periodical::Month => DEFAULT_MONTH,
                Periodical::Year => DEFAULT_YEAR,
            });
        let name = Local::now().format(fmt);

        format!("{name}.md")
    }
}

#[cfg(test)]
mod tests {
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
}
