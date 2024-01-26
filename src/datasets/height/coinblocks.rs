use std::fs;

use crate::structs::{AnyHeightMap, HeightMap};

use super::{AnyHeightDataset, ProcessedBlockData};

pub struct CoinblocksDataset {
    pub coinblocks_destroyed: HeightMap<f64>,
}

impl CoinblocksDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/coinblocks");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}.json");

        Ok(Self {
            coinblocks_destroyed: HeightMap::new(&f("destroyed")),
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
        self.coinblocks_destroyed
            .insert(height, coinblocks_destroyed);
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.coinblocks_destroyed]
    }
}
