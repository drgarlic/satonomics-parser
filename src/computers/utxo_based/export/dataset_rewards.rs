use crate::structs::HeightMap;

use super::{dataset::Dataset, DatasetInsertData};

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

impl Dataset for RewardsDataset {
    fn insert(&self, insert_data: &DatasetInsertData) {
        let &DatasetInsertData {
            height,
            coinbase,
            fees,
            ..
        } = insert_data;

        let subsidy = coinbase - fees;

        self.height_to_fees.insert(height, fees);
        self.height_to_subsidy.insert(height, subsidy);
    }

    fn get_min_last_height(&self) -> Option<usize> {
        [
            &self.height_to_fees.get_last_height(),
            &self.height_to_subsidy.get_last_height(),
        ]
        .iter()
        .min()
        .and_then(|opt| **opt)
    }

    fn get_min_initial_first_unsafe_height(&self) -> Option<usize> {
        [
            &self.height_to_fees.initial_first_unsafe_height,
            &self.height_to_subsidy.initial_first_unsafe_height,
        ]
        .iter()
        .min()
        .and_then(|opt| **opt)
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.height_to_fees.export()?;
        self.height_to_subsidy.export()?;

        Ok(())
    }
}
