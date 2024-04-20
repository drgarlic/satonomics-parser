use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, BiMap},
    states::RealizedState,
};

/// TODO: Fix fees not taken into account ?
pub struct RealizedSubDataset {
    min_initial_state: MinInitialState,

    realized_profit: BiMap<f32>,
    realized_loss: BiMap<f32>,
}

impl RealizedSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            realized_profit: BiMap::new_bin(1, &f("realized_profit")),
            realized_loss: BiMap::new_bin(1, &f("realized_loss")),
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
            date_blocks_range,
            ..
        }: &ProcessedBlockData,
        height_state: &RealizedState,
    ) {
        self.realized_profit
            .height
            .insert(height, height_state.realized_profit);

        self.realized_loss
            .height
            .insert(height, height_state.realized_loss);

        if is_date_last_block {
            self.realized_profit
                .date_insert_sum_range(date, date_blocks_range);

            self.realized_loss
                .date_insert_sum_range(date, date_blocks_range);
        }
    }
}

impl AnyDataset for RealizedSubDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.realized_loss, &self.realized_profit]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.realized_loss, &mut self.realized_profit]
    }
}
