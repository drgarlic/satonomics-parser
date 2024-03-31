use chrono::{NaiveDate, TimeZone, Utc};

pub const ONE_DAY_IN_DAYS: usize = 1;
pub const ONE_WEEK_IN_DAYS: usize = 7;
pub const TWO_WEEK_IN_DAYS: usize = 2 * ONE_WEEK_IN_DAYS;
pub const ONE_MONTH_IN_DAYS: usize = 30;
pub const THREE_MONTHS_IN_DAYS: usize = 3 * ONE_MONTH_IN_DAYS;
pub const ONE_YEAR_IN_DAYS: usize = 365;

pub fn timestamp_to_naive_date(timestamp: u32) -> NaiveDate {
    Utc.timestamp_opt(i64::from(timestamp), 0)
        .unwrap()
        .date_naive()
}

// pub fn string_to_naive_date(date: &str) -> NaiveDate {
//     NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
// }
