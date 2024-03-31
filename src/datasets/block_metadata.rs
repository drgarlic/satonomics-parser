use crate::{
    datasets::AnyDataset,
    parse::{AnyExportableMap, AnyHeightMap, HeightMap, WNaiveDate},
    utils::timestamp_to_naive_date,
};

use super::{GenericDataset, MinInitialState, ProcessedBlockData};

pub struct BlockMetadataDataset {
    min_initial_state: MinInitialState,

    pub date: HeightMap<WNaiveDate>,
    pub timestamp: HeightMap<u32>,
}

impl BlockMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            date: HeightMap::new_on_disk_bin(&f("date")),
            timestamp: HeightMap::new_on_disk_bin(&f("timestamp")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl GenericDataset for BlockMetadataDataset {
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
}

impl AnyDataset for BlockMetadataDataset {
    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.date, &self.timestamp]
    }

    fn to_any_exported_height_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        vec![&self.date, &self.timestamp]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
