use crate::{
    structs::{AnyHeightMap, HeightMap},
    traits::HeightDataset,
};

use super::DatasetInsertData;

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

impl<'a> HeightDataset<DatasetInsertData<'a>> for CoinblocksDataset {
    fn insert(&self, insert_data: &DatasetInsertData) {
        let &DatasetInsertData {
            height,
            coinblocks_destroyed,
            ..
        } = insert_data;

        self.height_to_coinblocks_destroyed
            .insert(height, coinblocks_destroyed);
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.height_to_coinblocks_destroyed]
    }
}
