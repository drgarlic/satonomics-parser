use crate::structs::{AnyHeightMap, HeightMap};

use super::{HeightDatasetTrait, ProcessedData};

pub struct RewardsDataset {
    pub height_to_fees: HeightMap<u64>,
    pub height_to_subsidy: HeightMap<u64>,
}

impl RewardsDataset {
    pub fn import(path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{path}/rewards/height_to_{s}.json");

        Ok(Self {
            height_to_fees: HeightMap::new(&f("fees")),
            height_to_subsidy: HeightMap::new(&f("subsidy")),
        })
    }
}

impl HeightDatasetTrait for RewardsDataset {
    fn insert(&self, processed_data: &ProcessedData) {
        let &ProcessedData {
            height,
            coinbase,
            fees,
            ..
        } = processed_data;

        let subsidy = coinbase - fees;

        self.height_to_fees.insert(height, fees);
        self.height_to_subsidy.insert(height, subsidy);
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.height_to_fees, &self.height_to_subsidy]
    }
}
