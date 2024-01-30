use std::fs;

use crate::structs::{AnyHeightMap, HeightMap};

use super::{AnyHeightDataset, ProcessedBlockData};

pub struct CoinblocksDataset {
    pub destroyed: HeightMap<f64>,
}

impl CoinblocksDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/coinblocks");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            destroyed: HeightMap::new_on_disk_bin(&f("destroyed")),
        })
    }
}

impl AnyHeightDataset for CoinblocksDataset {
    fn insert(
        &self,
        &ProcessedBlockData {
            height,
            coinblocks_destroyed,
            ..
        }: &ProcessedBlockData,
    ) {
        self.destroyed.insert(height, coinblocks_destroyed);
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.destroyed]
    }
}
