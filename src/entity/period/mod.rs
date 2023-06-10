#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterimType {
    CalendarMonth,
    FourWeek,
    FourFourFiveWeek,
}

pub mod accounting_period {
    use chrono::{Datelike, NaiveDate};
    use chronoutil::RelativeDuration;

    use crate::{
        domain::{
            ids::{InterimPeriodId, PeriodId},
            LedgerId,
        },
        orm::AccountRepository,
    };

    use super::{interim_accounting_period, InterimType};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_id: LedgerId,
        pub fiscal_year: i32,
        pub period_start: NaiveDate,
        pub period_type: InterimType,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub id: PeriodId,
        pub ledger_id: LedgerId,
        pub fiscal_year: i32,
        pub period_start: NaiveDate,
        pub period_end: NaiveDate,
        pub period_type: InterimType,
    }

    impl ActiveModel {
        pub fn create_interim_calendar(
            &self,
            orm: &dyn AccountRepository<
                interim_accounting_period::Model,
                interim_accounting_period::ActiveModel,
                InterimPeriodId,
            >,
        ) -> Result<Vec<interim_accounting_period::ActiveModel>, String> {
            let mut periods = Vec::<interim_accounting_period::ActiveModel>::new();
            let delta = RelativeDuration::months(1);
            let mut start = self.period_start;
            for _ in 1..=12 {
                let last_day = Self::get_days_in_month(start.year(), start.month())?;
                let end = NaiveDate::from_ymd_opt(start.year(), start.month(), last_day)
                    .ok_or(String::from("unable to calculate interim period"))?;
                let period = interim_accounting_period::Model {
                    start,
                    end,
                    parent_id: self.id,
                };
                let period = orm
                    .create(&period)
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
}

pub mod interim_accounting_period {
    use chrono::NaiveDate;

    use crate::domain::ids::{InterimPeriodId, PeriodId};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub parent_id: PeriodId,
        pub start: NaiveDate,
        pub end: NaiveDate,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub id: InterimPeriodId,
        pub parent_id: PeriodId,
        pub start: NaiveDate,
        pub end: NaiveDate,
    }
}
