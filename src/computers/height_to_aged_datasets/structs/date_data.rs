use std::{cell::RefCell, rc::Rc};

use chrono::{Datelike, NaiveDate};
use itertools::Itertools;

use super::{BlockData, SquashedBlockData};

pub struct DateData {
    pub date: NaiveDate,
    pub blocks: RefCell<Vec<Rc<BlockData>>>,
}

pub struct SquashedDateData {
    pub year: i32,
    pub blocks: Vec<SquashedBlockData>,
}

impl DateData {
    pub fn squash(&self) -> SquashedDateData {
        SquashedDateData {
            year: self.date.year(),
            blocks: self
                .blocks
                .borrow()
                .iter()
                .map(|block_data| block_data.squash())
                .collect_vec(),
        }
    }
}
