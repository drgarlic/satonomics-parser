use crate::{
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, AnyHeightMap, BiMap},
};

pub struct OutputSubDataset {
    min_initial_state: MinInitialState,

    pub count: BiMap<f32>,
    pub volume: BiMap<f32>,
}

impl OutputSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            count: BiMap::new_bin(&f("output_count")),
            volume: BiMap::new_bin(&f("output_volume")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }

    pub fn insert(
        &self,
        &ProcessedBlockData { height, .. }: &ProcessedBlockData,
        state: &OutputState,
    ) {
        self.count.height.insert(height, state.count);

        self.volume.height.insert(height, state.volume);
    }
}

impl AnyDataset for OutputSubDataset {
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

// ---
// STATE
// ---

#[derive(Debug, Default)]
pub struct OutputState {
    pub count: f32,
    pub volume: f32,
}

impl OutputState {
    pub fn iterate(&mut self, count: f32, volume: f32) {
        self.count += count;
        self.volume += volume;
    }
}
