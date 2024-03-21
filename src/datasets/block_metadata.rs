use chrono::NaiveDate;

use crate::{
    datasets::AnyDataset,
    parse::{AnyHeightMap, HeightMap, WNaiveDate},
    utils::timestamp_to_naive_date,
};

use super::ProcessedBlockData;

pub struct BlockMetadataDataset {
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,

    pub date: HeightMap<WNaiveDate>,
    pub timestamp: HeightMap<u32>,
}

impl BlockMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,
            date: HeightMap::new_on_disk_bin(&f("date")),
            timestamp: HeightMap::new_on_disk_bin(&f("timestamp")),
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
    }
}

impl AnyDataset for BlockMetadataDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height, timestamp, ..
        }: &ProcessedBlockData,
    ) {
        self.timestamp.insert(height, timestamp);

        self.date
            .insert(height, WNaiveDate::wrap(timestamp_to_naive_date(timestamp)));
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.date, &self.timestamp]
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }
}
