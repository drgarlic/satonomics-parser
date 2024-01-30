use std::{collections::BTreeMap, thread};

mod _trait;
mod block;

pub use _trait::*;
use block::*;
use chrono::NaiveDate;

pub struct ProcessedDateData {
    pub block_count: usize,
    pub first_height: usize,
    pub height: usize,
    pub date: NaiveDate,
}

pub struct DateDatasets {
    block: BlockDataset,
}

impl DateDatasets {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let path = format!("{parent_path}/date");

        thread::scope(|scope| {
            let block_handle = scope.spawn(|| BlockDataset::import(&path));

            Ok(Self {
                block: block_handle.join().unwrap()?,
            })
        })
    }

    pub fn get_date_to_last_height(&self) -> BTreeMap<String, usize> {
        self.block.last_height.import()
    }
}

impl AnyDateDatasets for DateDatasets {
    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateDataset + Send + Sync)> {
        vec![&self.block]
    }
}
