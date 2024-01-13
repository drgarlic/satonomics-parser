use crate::{
    structs::{AnyHeightMap, HeightMap},
    traits::HeightDataset,
};

use super::ProcessedData;

pub struct CoinblocksDataset {
    pub height_to_coinblocks_destroyed: HeightMap<f64>,
}

impl CoinblocksDataset {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self {
            height_to_coinblocks_destroyed: HeightMap::new("height_to_coinblocks_destroyed.json"),
        })
    }
}

impl<'a> HeightDataset<ProcessedData<'a>> for CoinblocksDataset {
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
