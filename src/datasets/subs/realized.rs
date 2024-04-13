use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, AnyHeightMap, BiMap},
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

            realized_profit: BiMap::new_bin(&f("realized_profit")),
            realized_loss: BiMap::new_bin(&f("realized_loss")),
        };

        s.min_initial_state
            .eat(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &self,
        &ProcessedBlockData { height, .. }: &ProcessedBlockData,
        height_state: &RealizedState,
    ) {
        self.realized_profit
            .height
            .insert(height, height_state.realized_profit);

        self.realized_loss
            .height
            .insert(height, height_state.realized_loss);
    }
}

impl AnyDataset for RealizedSubDataset {
    // fn compute(
    //     &self,
    //     &ExportData {
    //         convert_sum_heights_to_date,
    //         ..
    //     }: &ExportData,
    // ) {
    // self.realized_loss.compute_date(convert_sum_heights_to_date);
    // self.realized_profit
    //     .compute_date(convert_sum_heights_to_date);
    // }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.realized_loss.height, &self.realized_profit.height]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.realized_loss, &self.realized_profit]
    }
}
