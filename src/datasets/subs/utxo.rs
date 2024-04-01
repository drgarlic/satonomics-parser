use crate::{
    datasets::{AnyDataset, ExportData, MinInitialState, ProcessedBlockData},
    parse::{AnyExportableMap, AnyHeightMap, BiMap},
};

pub struct UTXOSubDataset {
    min_initial_state: MinInitialState,

    count: BiMap<usize>,
}

impl UTXOSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            count: BiMap::new_on_disk_bin(&f("utxo_count")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }

    pub fn insert(
        &self,
        &ProcessedBlockData { height, .. }: &ProcessedBlockData,
        state: &UTXOState,
    ) {
        self.count.height.insert(height, state.count);
    }
}

impl AnyDataset for UTXOSubDataset {
    fn compute(
        &self,
        &ExportData {
            convert_last_height_to_date,
            ..
        }: &ExportData,
    ) {
        self.count.compute_date(convert_last_height_to_date);
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count.height]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        vec![&self.count]
    }
}

// ---
// STATE
// ---

#[derive(Debug, Default)]
pub struct UTXOState {
    pub count: usize,
}

impl UTXOState {
    pub fn increment(&mut self, utxo_count: usize) {
        self.count += utxo_count;
    }

    pub fn decrement(&mut self, utxo_count: usize) {
        self.count -= utxo_count;
    }
}
