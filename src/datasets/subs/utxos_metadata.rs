use chrono::NaiveDate;

use crate::{
    datasets::ProcessedBlockData,
    parse::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap},
};

pub struct UTXOsMetadataSubDataset {
    count: BiMap<usize>,
}

#[derive(Debug, Default)]
pub struct UTXOsMetadataState {
    pub count: usize,
}

impl UTXOsMetadataState {
    pub fn increment(&mut self, utxo_count: usize) {
        self.count += utxo_count;
    }

    pub fn decrement(&mut self, utxo_count: usize) {
        self.count -= utxo_count;
    }
}

impl UTXOsMetadataSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        Ok(Self {
            count: BiMap::new_on_disk_bin(&f("utxo_count")),
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
        state: &UTXOsMetadataState,
    ) {
        self.count.height.insert(height, state.count);

        if is_date_last_block {
            self.count.date.insert(date, state.count);
        }
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count.height]
    }

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.count.date]
    }

    pub fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool {
        self.count.are_date_and_height_safe(date, height)
    }
}
