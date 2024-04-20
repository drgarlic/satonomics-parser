use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, BiMap},
    states::OutputState,
};

pub struct OutputSubDataset {
    min_initial_state: MinInitialState,

    pub count: BiMap<f32>,
    pub volume: BiMap<f32>,
}

impl OutputSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            count: BiMap::new_bin(1, &f("output_count")),
            volume: BiMap::new_bin(1, &f("output_volume")),
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
        state: &OutputState,
    ) {
        let count = self.count.height.insert(height, state.count);

        self.volume.height.insert(height, state.volume);

        if is_date_last_block {
            self.count.date.insert(date, count);

            self.volume.date_insert_sum_range(date, date_blocks_range);
        }
    }
}

impl AnyDataset for OutputSubDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.count, &self.volume]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.count, &mut self.volume]
    }
}
