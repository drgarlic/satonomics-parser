use crate::{
    bitcoin::sats_to_btc,
    datasets::AnyDataset,
    parse::{AnyBiMap, AnyHeightMap, BiMap},
};

use super::{GenericDataset, MinInitialState, ProcessedBlockData};

pub struct CoindaysDataset {
    min_initial_state: MinInitialState,

    pub destroyed: BiMap<f32>,
}

impl CoindaysDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            destroyed: BiMap::new_bin(&f("coindays_destroyed")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl GenericDataset for CoindaysDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height,
            satdays_destroyed,
            ..
        }: &ProcessedBlockData,
    ) {
        self.destroyed
            .height
            .insert(height, sats_to_btc(satdays_destroyed));
    }
}

impl AnyDataset for CoindaysDataset {
    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.destroyed.height]
    }

    // fn compute(
    //     &self,
    //     &ExportData {
    //         convert_sum_heights_to_date,
    //         ..
    //     }: &ExportData,
    // ) {
    // self.destroyed.compute_date(convert_sum_heights_to_date);
    // }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.destroyed]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
