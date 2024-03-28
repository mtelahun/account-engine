use chrono::NaiveDate;
use postgres_types::{FromSql, ToSql};

use crate::shared_kernel::ids::{InterimPeriodId, PeriodId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "interimtype")]
pub enum InterimType {
    #[postgres(name = "calendar_month")]
    CalendarMonth,
    #[postgres(name = "4week")]
    FourWeek,
    #[postgres(name = "445week")]
    FourFourFiveWeek,
}

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

#[cfg(test)]
mod tests {
    use super::InterimType;

    #[test]
    fn test_interimtype_to_string() {
        let calendar_month = InterimType::CalendarMonth;
        assert_eq!(
            calendar_month.to_string(),
            "calendar_month",
            "Enum -> string is correct"
        );

        let calendar_month = InterimType::FourWeek;
        assert_eq!(
            calendar_month.to_string(),
            "4week",
            "Enum -> string is correct"
        );

        let calendar_month = InterimType::FourFourFiveWeek;
        assert_eq!(
            calendar_month.to_string(),
            "445week",
            "Enum -> string is correct"
        );
    }
}
