use chrono::NaiveDate;

use super::BlockData;

pub struct DateData {
    pub date: NaiveDate,
    pub blocks: Vec<BlockData>,
}
