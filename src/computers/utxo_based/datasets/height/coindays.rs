use crate::structs::{AnyHeightMap, HeightMap};

use super::{HeightDatasetTrait, ProcessedData};

pub struct CoindaysDataset {
    pub height_to_coindays_destroyed: HeightMap<f64>,
}

impl CoindaysDataset {
    pub fn import(path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{path}/coindays/height_to_{s}.json");

        Ok(Self {
            height_to_coindays_destroyed: HeightMap::new(&f("coindays_destroyed")),
        })
    }
}

impl HeightDatasetTrait for CoindaysDataset {
    fn insert(&self, processed_data: &ProcessedData) {
        let &ProcessedData {
            height,
            coindays_destroyed,
            ..
        } = processed_data;

        self.height_to_coindays_destroyed
            .insert(height, coindays_destroyed);
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.height_to_coindays_destroyed]
    }
}
