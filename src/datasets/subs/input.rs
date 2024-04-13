use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, AnyHeightMap, BiMap},
    states::InputState,
};

pub struct InputSubDataset {
    min_initial_state: MinInitialState,

    pub count: BiMap<f32>,
    pub volume: BiMap<f32>,
}

impl InputSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            count: BiMap::new_bin(&f("input_count")),
            volume: BiMap::new_bin(&f("input_volume")),
        };

        s.min_initial_state
            .eat(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &self,
        &ProcessedBlockData { height, .. }: &ProcessedBlockData,
        state: &InputState,
    ) {
        self.count.height.insert(height, state.count);

        self.volume.height.insert(height, state.volume);
    }
}

impl AnyDataset for InputSubDataset {
    // fn compute(
    //     &self,
    //     &ExportData {
    //         convert_last_height_to_date,
    //         convert_sum_heights_to_date,
    //         ..
    //     }: &ExportData,
    // ) {
    // self.count.compute_date(convert_last_height_to_date);
    // self.volume.compute_date(convert_sum_heights_to_date);
    // }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count.height, &self.volume.height]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.count, &self.volume]
    }
}
