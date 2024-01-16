use bincode::{Decode, Encode};
use chrono::NaiveDate;

use super::{BlockData, WNaiveDate};

#[derive(Encode, Decode, Debug)]
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
