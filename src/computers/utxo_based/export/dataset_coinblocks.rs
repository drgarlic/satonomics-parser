use crate::structs::HeightMap;

use super::{dataset::Dataset, DatasetInsertData};

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

impl Dataset for CoinblocksDataset {
    fn insert(&self, insert_data: &DatasetInsertData) {
        let &DatasetInsertData {
            height,
            coinblocks_destroyed,
            ..
        } = insert_data;

        self.height_to_coinblocks_destroyed
            .insert(height, coinblocks_destroyed);
    }

    fn get_min_last_height(&self) -> Option<usize> {
        [&self.height_to_coinblocks_destroyed.get_last_height()]
            .iter()
            .min()
            .and_then(|opt| **opt)
    }

    fn get_min_initial_first_unsafe_height(&self) -> Option<usize> {
        [&self
            .height_to_coinblocks_destroyed
            .initial_first_unsafe_height]
        .iter()
        .min()
        .and_then(|opt| **opt)
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.height_to_coinblocks_destroyed.export()?;

        Ok(())
    }
}
