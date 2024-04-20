use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, BiMap},
    states::UTXOState,
};

pub struct UTXOSubDataset {
    min_initial_state: MinInitialState,

    count: BiMap<usize>,
}

impl UTXOSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            count: BiMap::new_bin(1, &f("utxo_count")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &ProcessedBlockData {
            height,
            is_date_last_block,
            date,
            ..
        }: &ProcessedBlockData,
        state: &UTXOState,
    ) {
        let count = self.count.height.insert(height, state.count);

        if is_date_last_block {
            self.count.date.insert(date, count);
        }
    }
}

impl AnyDataset for UTXOSubDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.count]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.count]
    }
}
