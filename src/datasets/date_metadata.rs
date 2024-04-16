use crate::{
    datasets::AnyDataset,
    parse::{AnyDateMap, DateMap},
};

use super::{GenericDataset, MinInitialState, ProcessedBlockData};

pub struct DateMetadataDataset {
    min_initial_state: MinInitialState,

    pub first_height: DateMap<usize>,
    pub last_height: DateMap<usize>,
}

impl DateMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            first_height: DateMap::new_bin(&f("first_height")),
            last_height: DateMap::new_bin(&f("last_height")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }
}

impl GenericDataset for DateMetadataDataset {
    fn insert_data(
        &self,
        &ProcessedBlockData {
            date,
            date_first_height,
            height,
            ..
        }: &ProcessedBlockData,
    ) {
        self.first_height.insert(date, date_first_height);

        self.last_height.insert(date, height);
    }
}

impl AnyDataset for DateMetadataDataset {
    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.first_height, &self.last_height]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
