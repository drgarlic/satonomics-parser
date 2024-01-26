use std::fs;

use chrono::NaiveDate;

use crate::{
    structs::{AnyHeightMap, HeightMap},
    utils::timestamp_to_naive_date,
};

use super::{AnyHeightDataset, ProcessedBlockData};

pub struct TimeDataset {
    pub date: HeightMap<NaiveDate>,
    pub timestamp: HeightMap<u32>,
}

impl TimeDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/time");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}.json");

        Ok(Self {
            date: HeightMap::new(&f("date")),
            timestamp: HeightMap::new(&f("timestamp")),
        })
    }
}

impl AnyHeightDataset for TimeDataset {
    fn insert(
        &self,
        &ProcessedBlockData {
            height, timestamp, ..
        }: &ProcessedBlockData,
    ) {
        self.timestamp.insert(height, timestamp);

        self.date.insert(height, timestamp_to_naive_date(timestamp));
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.date, &self.timestamp]
    }
}
