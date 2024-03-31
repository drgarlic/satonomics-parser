use crate::{
    bitcoin::sats_to_btc,
    datasets::{AnyDataset, ExportData, MinInitialState, ProcessedBlockData},
    parse::{AnyExportableMap, AnyHeightMap, BiMap},
};

pub struct SupplySubDataset {
    min_initial_state: MinInitialState,

    total_supply: BiMap<f32>,
}

impl SupplySubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            total_supply: BiMap::new_on_disk_bin(&f("total_supply")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }

    pub fn insert(
        &self,
        &ProcessedBlockData { height, .. }: &ProcessedBlockData,
        state: &SupplyState,
    ) {
        self.total_supply
            .height
            .insert(height, sats_to_btc(state.total_supply));
    }
}

impl AnyDataset for SupplySubDataset {
    fn compute(
        &mut self,
        &ExportData {
            last_height_to_date,
            ..
        }: &ExportData,
    ) {
        self.total_supply.compute_date(last_height_to_date);
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.total_supply.height]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        vec![&self.total_supply]
    }
}

// ---
// STATE
// ---

#[derive(Debug, Default)]
pub struct SupplyState {
    pub total_supply: u64,
}

impl SupplyState {
    pub fn increment(&mut self, amount: u64) {
        self.total_supply += amount;
    }

    pub fn decrement(&mut self, amount: u64) {
        self.total_supply -= amount;
    }
}
