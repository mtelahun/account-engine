use chrono::{Datelike, NaiveDate};
use chronoutil::RelativeDuration;

use super::AccountError;

#[derive(Clone, Debug)]
pub struct AccountingPeriod {
    pub fiscal_year: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub periods: Vec<InterimPeriod>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InterimPeriod {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[derive(Clone, Copy, Debug)]
pub enum InterimType {
    CalendarMonth,
    FourWeek,
    FourFourFiveWeek,
}

impl AccountingPeriod {
    pub fn create_interim_calendar(&self) -> Result<Vec<InterimPeriod>, AccountError> {
        let mut periods = Vec::<InterimPeriod>::new();
        let delta = RelativeDuration::months(1);
        let mut start = self.period_start;
        for _ in 1..=12 {
            let last_day = AccountingPeriod::get_days_in_month(start.year(), start.month())?;
            let end = NaiveDate::from_ymd_opt(start.year(), start.month(), last_day).ok_or(
                AccountError::Internal("unable to calculate interim period".into()),
            )?;
            let period = InterimPeriod { start, end };
            periods.push(period);

            start = start + delta;
        }

        Ok(periods)
    }

    fn get_days_in_month(year: i32, month: u32) -> Result<u32, AccountError> {
        let this_month = NaiveDate::from_ymd_opt(year, month, 1).ok_or(AccountError::Internal(
            "could not calculate first of the month".into(),
        ))?;
        let next_month = NaiveDate::from_ymd_opt(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        )
        .ok_or(AccountError::Internal(
            "could not calculate next month".into(),
        ))?;

        Ok(next_month.signed_duration_since(this_month).num_days() as u32)
    }
}
