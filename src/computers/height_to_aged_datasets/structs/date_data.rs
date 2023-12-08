use std::sync::{Arc, RwLock};

use chrono::NaiveDate;

use super::BlockData;

pub struct DateData {
    pub date: NaiveDate,
    pub blocks: RwLock<Vec<Arc<BlockData>>>,
}
