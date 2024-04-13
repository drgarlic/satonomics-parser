use chrono::{NaiveDate, TimeZone, Utc};

pub const ONE_DAY_IN_DAYS: usize = 1;
pub const ONE_WEEK_IN_DAYS: usize = 7;
pub const TWO_WEEK_IN_DAYS: usize = 2 * ONE_WEEK_IN_DAYS;
pub const ONE_MONTH_IN_DAYS: usize = 30;
pub const THREE_MONTHS_IN_DAYS: usize = 3 * ONE_MONTH_IN_DAYS;
pub const ONE_YEAR_IN_DAYS: usize = 365;

pub const MILLISECONDS_IN_MINUTE: usize = 1000 * 60;
pub const MILLISECONDS_IN_HOUR: usize = 60 * MILLISECONDS_IN_MINUTE;
pub const MILLISECONDS_IN_DAY: usize = 24 * MILLISECONDS_IN_HOUR;
pub const MILLISECONDS_IN_YEAR: usize = 365 * MILLISECONDS_IN_DAY;
pub const TIMESTAMP_STARTING_YEAR: usize = 1970;

pub fn timestamp_to_naive_date(timestamp: u32) -> NaiveDate {
    Utc.timestamp_opt(i64::from(timestamp), 0)
        .unwrap()
        .date_naive()
}

pub fn timestamp_to_year(timestamp: u32) -> u32 {
    ((timestamp as usize / MILLISECONDS_IN_YEAR) + TIMESTAMP_STARTING_YEAR) as u32
}
