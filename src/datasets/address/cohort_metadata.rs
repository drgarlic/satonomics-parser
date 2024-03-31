use crate::{
    datasets::{AnyDataset, ExportData, MinInitialState},
    parse::{AnyExportableMap, AnyHeightMap, BiMap},
};

pub struct MetadataDataset {
    min_initial_state: MinInitialState,

    pub address_count: BiMap<usize>,
}

impl MetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            address_count: BiMap::new_on_disk_bin(&f("address_count")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl AnyDataset for MetadataDataset {
    fn compute(
        &mut self,
        &ExportData {
            last_height_to_date,
            ..
        }: &ExportData,
    ) {
        self.address_count.compute_date(last_height_to_date);
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.address_count.height]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        vec![&self.address_count]
    }
}
