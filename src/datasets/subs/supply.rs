use crate::{
    bitcoin::sats_to_btc,
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, BiMap},
    states::SupplyState,
};

pub struct SupplySubDataset {
    min_initial_state: MinInitialState,

    pub total: BiMap<f32>,
}

impl SupplySubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            total: BiMap::new_bin(1, &f("supply")),
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
        state: &SupplyState,
    ) {
        let total_supply = self.total.height.insert(height, sats_to_btc(state.supply));

        if is_date_last_block {
            self.total.date.insert(date, total_supply);
        }
    }
}

impl AnyDataset for SupplySubDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.total]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.total]
    }
}
