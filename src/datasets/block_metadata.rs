use crate::{
    datasets::AnyDataset,
    parse::{AnyHeightMap, HeightMap, WNaiveDate},
    utils::timestamp_to_naive_date,
};

use super::{MinInitialState, ProcessedBlockData};

pub struct BlockMetadataDataset {
    min_initial_state: MinInitialState,

    pub date: HeightMap<WNaiveDate>,
    pub timestamp: HeightMap<u32>,
}

impl BlockMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            date: HeightMap::new_bin(1, &f("date")),
            timestamp: HeightMap::new_bin(1, &f("timestamp")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert_data(
        &mut self,
        &ProcessedBlockData {
            height, timestamp, ..
        }: &ProcessedBlockData,
    ) {
        self.timestamp.insert(height, timestamp);

        self.date
            .insert(height, WNaiveDate::wrap(timestamp_to_naive_date(timestamp)));
    }
}

impl AnyDataset for BlockMetadataDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.date, &self.timestamp]
    }

    fn to_any_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![&mut self.date, &mut self.timestamp]
    }
}
