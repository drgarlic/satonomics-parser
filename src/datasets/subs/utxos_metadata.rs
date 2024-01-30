use crate::{
    datasets::ProcessedBlockData,
    structs::{AnyDateMap, AnyHeightMap, BiMap},
};

pub struct UTXOsMetadataSubDataset {
    count: BiMap<usize>,
}

impl UTXOsMetadataSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/utxos");
        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            count: BiMap::new_on_disk_bin(&f("count")),
        })
    }

    pub fn insert(
        &self,
        &ProcessedBlockData {
            date,
            height,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
        utxo_count: usize,
    ) {
        self.count.height.insert(height, utxo_count);

        if is_date_last_block {
            self.count.date.insert(date, utxo_count);
        }
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count.height]
    }

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.count.date]
    }
}
