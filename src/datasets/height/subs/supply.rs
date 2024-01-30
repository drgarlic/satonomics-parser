use std::fs;

use crate::structs::{AnyHeightMap, HeightMap};

pub struct SupplyDataset {
    total: HeightMap<u64>,
}

impl SupplyDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/supply");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            total: HeightMap::new_on_disk_bin(&f("total")),
        })
    }

    pub fn insert(&self, height: usize, total_supply: u64) {
        self.total.insert(height, total_supply);
    }

    pub fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.total]
    }
}
