use crate::{
    datasets::AnyDataset,
    structs::{AnyDateMap, DateMap},
};

use super::ProcessedDateData;

pub struct DateMetadataDataset {
    pub first_height: DateMap<usize>,
    pub last_height: DateMap<usize>,
    pub block_count: DateMap<usize>,
}

impl DateMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/date_metadata");

        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            first_height: DateMap::new_on_disk_bin(&f("first_height")),
            last_height: DateMap::new_in_memory_bin(&f("last_height")),
            block_count: DateMap::new_on_disk_bin(&f("block_count")),
        })
    }
}

impl AnyDataset for DateMetadataDataset {
    fn insert_date_data(
        &self,
        &ProcessedDateData {
            date,
            first_height,
            height,
            ..
        }: &ProcessedDateData,
    ) {
        self.first_height.insert(date, first_height);

        self.last_height.insert(date, height);

        self.block_count.insert(date, height + 1 - first_height);
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.block_count, &self.first_height, &self.last_height]
    }
}
