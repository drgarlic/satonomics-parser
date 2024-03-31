use crate::{
    datasets::AnyDataset,
    parse::{AnyDateMap, AnyExportableMap, DateMap},
};

use super::{GenericDataset, MinInitialState, ProcessedDateData};

pub struct DateMetadataDataset {
    min_initial_state: MinInitialState,

    pub first_height: DateMap<usize>,
    pub last_height: DateMap<usize>,
}

impl DateMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            first_height: DateMap::new_in_memory_bin(&f("first_height")),
            last_height: DateMap::new_in_memory_bin(&f("last_height")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl GenericDataset for DateMetadataDataset {
    fn insert_date_data(
        &self,
        &ProcessedDateData {
            date,
            first_height,
            height,
            ..
        }: &ProcessedDateData,
    ) {
        self.first_height.insert(date, first_height);

        self.last_height.insert(date, height);
    }
}

impl AnyDataset for DateMetadataDataset {
    fn to_any_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.first_height, &self.last_height]
    }

    fn to_any_exported_date_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        vec![&self.first_height, &self.last_height]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
