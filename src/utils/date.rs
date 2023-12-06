use chrono::{NaiveDate, TimeZone, Utc};

pub fn timestamp_to_naive_date(timestamp: u32) -> NaiveDate {
    Utc.timestamp_opt(i64::from(timestamp), 0)
        .unwrap()
        .date_naive()
}

pub fn string_to_naive_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}
