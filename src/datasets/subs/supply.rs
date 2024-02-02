use chrono::NaiveDate;

use crate::{
    datasets::ProcessedBlockData,
    structs::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap},
};

pub struct SupplySubDataset {
    total: BiMap<u64>,
}

impl SupplySubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/supply");
        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            total: BiMap::new_on_disk_bin(&f("total")),
        })
    }

    pub fn insert(
        &self,
        &ProcessedBlockData {
            date,
            height,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
        total_supply: u64,
    ) {
        self.total.height.insert(height, total_supply);

        if is_date_last_block {
            self.total.date.insert(date, total_supply);
        }
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.total.height]
    }

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.total.date]
    }

    pub fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool {
        self.total.are_date_and_height_safe(date, height)
    }
}
