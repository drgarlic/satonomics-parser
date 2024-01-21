use crate::structs::{AnyHeightMap, HeightMap};

use super::{HeightDatasetTrait, ProcessedData};

pub struct CoinblocksDataset {
    pub height_to_coinblocks_destroyed: HeightMap<f64>,
}

impl CoinblocksDataset {
    pub fn import(path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{path}/coinblocks/height_to_{s}.json");

        Ok(Self {
            height_to_coinblocks_destroyed: HeightMap::new(&f("coinblocks_destroyed")),
        })
    }
}

impl HeightDatasetTrait for CoinblocksDataset {
    fn insert(&self, processed_data: &ProcessedData) {
        let &ProcessedData {
            height,
            coinblocks_destroyed,
            ..
        } = processed_data;

        self.height_to_coinblocks_destroyed
            .insert(height, coinblocks_destroyed);
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.height_to_coinblocks_destroyed]
    }
}
