use chrono::{DateTime, Months, TimeDelta, TimeZone};

use super::*;

#[derive(Debug, Error)]
#[error(
    "Time math error. {0} with an interval of {1} produced an invalid DateTime struct."
)]
pub struct TimeMathError(Periodical, i64);

pub fn add_interval<Tz>(
    date_time: DateTime<Tz>,
    period: Periodical,
    interval: i64,
) -> Result<DateTime<Tz>, TimeMathError>
where
    Tz: TimeZone,
{
    // handle days or weeks
    if let Some(delta_time) = period.get_time_delta(interval) {
        return date_time
            .checked_add_signed(delta_time)
            .ok_or(TimeMathError(period, interval));
    }
    // handle months or years
    let mut abs = interval.unsigned_abs() as u32;
    if period == Periodical::Year {
        abs *= 12;
    }
    let new_date_time = match interval.is_positive() {
        false => date_time.checked_sub_months(Months::new(abs)),
        true => date_time.checked_add_months(Months::new(abs)),
    };
    new_date_time.ok_or(TimeMathError(period, interval))
}

/// provides time deltas for days and weeks.
/// TimeDelta's between months and years are variable,
/// thus need to be calculated via DateTime structs.
impl Periodical {
    fn get_time_delta(&self, interval: i64) -> Option<TimeDelta> {
        match self {
            Self::Day => Some(TimeDelta::days(interval)),
            Self::Week => Some(TimeDelta::weeks(interval)),
            Self::Month => None,
            Self::Year => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{Error, Result};
    use chrono::Utc;

    use super::*;

    /// test date Sunday 2004-02-29 00:00:00 UTC
    fn test_date() -> Result<DateTime<Utc>> {
        DateTime::from_timestamp(1078012800, 0)
            .ok_or(Error::msg("invalid time stamp passed to test case."))
    }

    #[test]
    fn test_day_math() -> Result<()> {
        let test_cases = [
            (1_i64, 1078099200),
            (2, 1078185600),
            (-1, 1077926400),
            (-2, 1077840000),
        ];

        test_cases.into_iter().try_for_each(|(interval, want)| {
            let got = add_interval(test_date()?, Periodical::Day, interval)?
                .timestamp();
            assert_eq!(
                want, got,
                "Test Periodical::Day, with an interval offet of {interval}."
            );
            anyhow::Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn test_week_math() -> Result<()> {
        let test_cases = [
            (-2_i64, 1076803200),
            (-1, 1077408000),
            (1, 1078617600),
            (2, 1079222400),
        ];

        test_cases.into_iter().try_for_each(|(interval, want)| {
            let got = add_interval(test_date()?, Periodical::Week, interval)?
                .timestamp();
            assert_eq!(
                want, got,
                "Test Periodical::Week with interval of {interval}."
            );

            anyhow::Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn test_month_math() -> Result<()> {
        let test_cases = [
            (-2_i64, 1072656000),
            (-1, 1075334400),
            (1, 1080518400),
            (2, 1083196800),
        ];

        test_cases.into_iter().try_for_each(|(interval, want)| {
            let got = add_interval(test_date()?, Periodical::Month, interval)?
                .timestamp();
            assert_eq!(
                want, got,
                "Test Periodical::Week with interval of {interval}."
            );

            anyhow::Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn test_year_math() -> Result<()> {
        let test_cases = [
            (-4_i64, 951782400),
            (-1, 1046390400),
            (1, 1109548800),
            (4, 1204243200),
        ];

        test_cases.into_iter().try_for_each(|(interval, want)| {
            let got = add_interval(test_date()?, Periodical::Year, interval)?
                .timestamp();
            assert_eq!(
                want, got,
                "Test Periodical::Year with interval of {interval}"
            );

            anyhow::Ok(())
        })?;

        Ok(())
    }
}
