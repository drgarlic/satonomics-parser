use crate::{
    bitcoin::sats_to_btc,
    datasets::{AnyDataset, ExportData, MinInitialState, ProcessedBlockData},
    parse::{AnyExportableMap, AnyHeightMap, BiMap},
};

pub struct SupplySubDataset {
    min_initial_state: MinInitialState,

    pub total: BiMap<f32>,
}

impl SupplySubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            total: BiMap::new_in_memory_bin(&f("supply")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }

    pub fn insert(
        &self,
        &ProcessedBlockData { height, .. }: &ProcessedBlockData,
        state: &SupplyState,
    ) {
        self.total.height.insert(height, sats_to_btc(state.supply));
    }
}

impl AnyDataset for SupplySubDataset {
    fn prepare(
        &self,
        &ExportData {
            convert_last_height_to_date,
            ..
        }: &ExportData,
    ) {
        self.total.compute_date(convert_last_height_to_date);
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.total.height]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        vec![&self.total]
    }
}

// ---
// STATE
// ---

#[derive(Debug, Default)]
pub struct SupplyState {
    pub supply: u64,
}

impl SupplyState {
    pub fn increment(&mut self, amount: u64) {
        self.supply += amount;
    }

    pub fn decrement(&mut self, amount: u64) {
        self.supply -= amount;
    }
}
