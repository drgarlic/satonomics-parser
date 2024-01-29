use std::fs;

use crate::structs::{AnyHeightMap, HeightMap};

use super::{AnyHeightDataset, ProcessedBlockData};

pub struct CoindaysDataset {
    pub destroyed: HeightMap<f64>,
}

impl CoindaysDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/coindays");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            destroyed: HeightMap::new_on_disk_bin(&f("destroyed")),
        })
    }
}

impl AnyHeightDataset for CoindaysDataset {
    fn insert(
        &self,
        &ProcessedBlockData {
            height,
            coindays_destroyed,
            ..
        }: &ProcessedBlockData,
    ) {
        self.destroyed.insert(height, coindays_destroyed);
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.destroyed]
    }
}
