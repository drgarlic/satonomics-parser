use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, BiMap},
};

pub struct MetadataDataset {
    min_initial_state: MinInitialState,

    address_count: BiMap<usize>,
}

impl MetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            address_count: BiMap::new_bin(1, &f("address_count")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &ProcessedBlockData {
            height,
            date,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
        address_count: usize,
    ) {
        self.address_count.height.insert(height, address_count);

        if is_date_last_block {
            self.address_count.date.insert(date, address_count);
        }
    }
}

impl AnyDataset for MetadataDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.address_count]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.address_count]
    }
}
