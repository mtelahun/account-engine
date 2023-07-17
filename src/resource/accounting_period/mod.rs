use chrono::{Datelike, NaiveDate};
use chronoutil::RelativeDuration;
use postgres_types::{FromSql, ToSql};

use crate::{
    domain::ids::{InterimPeriodId, PeriodId},
    repository::ResourceOperations,
};

pub mod interim_period;
use interim_period::InterimType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub fiscal_year: i32,
    pub period_start: NaiveDate,
    pub period_type: InterimType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct ActiveModel {
    pub id: PeriodId,
    pub fiscal_year: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub period_type: InterimType,
}

impl std::fmt::Display for InterimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            InterimType::CalendarMonth => "calendar_month",
            InterimType::FourWeek => "4week",
            InterimType::FourFourFiveWeek => "445week",
        };

        write!(f, "{s}")
    }
}

impl ActiveModel {
    pub(crate) async fn create_interim_calendar(
        &self,
        orm: &(dyn ResourceOperations<
            interim_period::Model,
            interim_period::ActiveModel,
            InterimPeriodId,
        > + Send
              + Sync),
    ) -> Result<Vec<interim_period::ActiveModel>, String> {
        let mut periods = Vec::<interim_period::ActiveModel>::new();
        let delta = RelativeDuration::months(1);
        let mut start = self.period_start;
        for _ in 1..=12 {
            let last_day = Self::get_days_in_month(start.year(), start.month())?;
            let end = NaiveDate::from_ymd_opt(start.year(), start.month(), last_day)
                .ok_or(String::from("unable to calculate interim period"))?;
            let period = interim_period::Model {
                start,
                end,
                parent_id: self.id,
            };
            let period = orm
                .insert(&period)
                .await
                .map_err(|e| format!("error creating interim period: {}", e))?;
            periods.push(period);

            start = start + delta;
        }

        Ok(periods)
    }

    fn get_days_in_month(year: i32, month: u32) -> Result<u32, String> {
        let this_month = NaiveDate::from_ymd_opt(year, month, 1)
            .ok_or(String::from("could not calculate first of the month"))?;
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
        .ok_or(String::from("could not calculate next month"))?;

        Ok(next_month.signed_duration_since(this_month).num_days() as u32)
    }
}
