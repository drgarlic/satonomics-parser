use chrono::NaiveDate;

use crate::{
    structs::{AnyHeightMap, HeightMap},
    utils::timestamp_to_naive_date,
};

use super::{HeightDatasetTrait, ProcessedData};

pub struct TimeDataset {
    pub height_to_date: HeightMap<NaiveDate>,
    pub height_to_timestamp: HeightMap<u32>,
}

impl TimeDataset {
    pub fn import(path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{path}/time/height_to_{s}.json");

        Ok(Self {
            height_to_date: HeightMap::new(&f("date")),
            height_to_timestamp: HeightMap::new(&f("timestamp")),
        })
    }
}

impl HeightDatasetTrait for TimeDataset {
    fn insert(&self, processed_data: &ProcessedData) {
        let &ProcessedData {
            height, timestamp, ..
        } = processed_data;

        self.height_to_timestamp.insert(height, timestamp);

        self.height_to_date
            .insert(height, timestamp_to_naive_date(timestamp));
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.height_to_date, &self.height_to_timestamp]
    }
}
