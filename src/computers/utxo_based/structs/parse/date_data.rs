use bincode::{Decode, Encode};
use chrono::NaiveDate;

use crate::structs::WNaiveDate;

use super::BlockData;

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
