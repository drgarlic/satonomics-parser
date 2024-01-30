use std::fs;

use crate::structs::{AnyHeightMap, HeightMap};

use super::{AnyHeightDataset, ProcessedBlockData};

pub struct RewardsDataset {
    pub fees: HeightMap<u64>,
    pub subsidy: HeightMap<u64>,
}

impl RewardsDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/rewards");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            fees: HeightMap::new_on_disk_bin(&f("fees")),
            subsidy: HeightMap::new_on_disk_bin(&f("subsidy")),
        })
    }
}

impl AnyHeightDataset for RewardsDataset {
    fn insert(
        &self,
        &ProcessedBlockData {
            height,
            coinbase,
            fees,
            ..
        }: &ProcessedBlockData,
    ) {
        let subsidy = coinbase - fees;

        self.fees.insert(height, fees);
        self.subsidy.insert(height, subsidy);
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.fees, &self.subsidy]
    }
}
