use chrono::NaiveDate;
use savefile_derive::Savefile;

use super::{BlockData, WNaiveDate};

#[derive(Savefile, Debug)]
pub struct DateData {
    pub date: WNaiveDate,
    pub blocks: Vec<BlockData>,
}

impl DateData {
    pub fn new(date: NaiveDate, blocks: Vec<BlockData>) -> Self {
        Self {
            date: WNaiveDate::wrap(date),
            blocks,
        }
    }
}
