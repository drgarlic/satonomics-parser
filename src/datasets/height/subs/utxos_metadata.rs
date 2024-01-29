use std::fs;

use crate::structs::{AnyHeightMap, HeightMap};

pub struct UTXOsMetadataDataset {
    count: HeightMap<usize>,
}

impl UTXOsMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/utxos");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            count: HeightMap::new_on_disk_bin(&f("count")),
        })
    }

    pub fn insert(&self, height: usize, utxo_count: usize) {
        self.count.insert(height, utxo_count);
    }

    pub fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count]
    }
}
