use crate::{
    bitcoin::sats_to_btc,
    datasets::AnyDataset,
    parse::{AnyBiMap, BiMap},
};

use super::{MinInitialState, ProcessedBlockData};

pub struct CoindaysDataset {
    min_initial_state: MinInitialState,

    pub destroyed: BiMap<f32>,
}

impl CoindaysDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            destroyed: BiMap::new_bin(1, &f("coindays_destroyed")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert_data(
        &mut self,
        &ProcessedBlockData {
            height,
            satdays_destroyed,
            date_blocks_range,
            is_date_last_block,
            date,
            ..
        }: &ProcessedBlockData,
    ) {
        self.destroyed
            .height
            .insert(height, sats_to_btc(satdays_destroyed));

        if is_date_last_block {
            self.destroyed
                .date_insert_sum_range(date, date_blocks_range)
        }
    }
}

impl AnyDataset for CoindaysDataset {
    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.destroyed]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.destroyed]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
