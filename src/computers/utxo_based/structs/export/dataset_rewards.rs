use crate::{
    structs::{AnyHeightMap, HeightMap},
    traits::HeightDataset,
};

use super::DatasetInsertedData;

pub struct RewardsDataset {
    pub height_to_fees: HeightMap<f64>,
    pub height_to_subsidy: HeightMap<f64>,
}

impl RewardsDataset {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self {
            height_to_fees: HeightMap::new("height_to_fees.json"),
            height_to_subsidy: HeightMap::new("height_to_subsidy.json"),
        })
    }
}

impl<'a> HeightDataset<DatasetInsertedData<'a>> for RewardsDataset {
    fn insert(&self, insert_data: &DatasetInsertedData) {
        let &DatasetInsertedData {
            height,
            coinbase,
            fees,
            ..
        } = insert_data;

        let subsidy = coinbase - fees;

        self.height_to_fees.insert(height, fees);
        self.height_to_subsidy.insert(height, subsidy);
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.height_to_fees, &self.height_to_subsidy]
    }
}
