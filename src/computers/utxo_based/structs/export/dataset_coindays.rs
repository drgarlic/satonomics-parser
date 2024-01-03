use crate::{
    structs::{AnyHeightMap, HeightMap},
    traits::HeightDataset,
};

use super::DatasetInsertedData;

pub struct CoindaysDataset {
    pub height_to_coindays_destroyed: HeightMap<f64>,
}

impl CoindaysDataset {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self {
            height_to_coindays_destroyed: HeightMap::new("height_to_coindays_destroyed.json"),
        })
    }
}

impl<'a> HeightDataset<DatasetInsertedData<'a>> for CoindaysDataset {
    fn insert(&self, insert_data: &DatasetInsertedData) {
        let &DatasetInsertedData {
            height,
            coindays_destroyed,
            ..
        } = insert_data;

        self.height_to_coindays_destroyed
            .insert(height, coindays_destroyed);
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.height_to_coindays_destroyed]
    }
}
