use chrono::{Datelike, NaiveDate};
use savefile_derive::Savefile;

use super::{BlockData, WNaiveDate};

#[derive(Savefile, Debug)]
pub struct DateData {
    pub date: WNaiveDate,
    pub index: u16,
    pub year: u16,
    pub blocks: Vec<BlockData>,
}

impl DateData {
    pub fn new(index: u16, date: NaiveDate, blocks: Vec<BlockData>) -> Self {
        Self {
            index,
            date: WNaiveDate::wrap(date),
            year: date.year() as u16,
            blocks,
        }
    }

    #[inline(always)]
    pub fn reverse_index(&self, len: u16) -> u16 {
        reverse_date_index(self.index, len)
    }
}

#[inline(always)]
pub fn reverse_date_index(date_index: u16, dates_len: u16) -> u16 {
    dates_len - 1 - date_index
}
