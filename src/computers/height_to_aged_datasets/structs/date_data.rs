use std::{cell::RefCell, rc::Rc};

use chrono::NaiveDate;

use super::BlockData;

pub struct DateData {
    pub date: NaiveDate,
    pub blocks: RefCell<Vec<Rc<BlockData>>>,
}
