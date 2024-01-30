use crate::{
    datasets::AnyDataset,
    structs::{AnyHeightMap, HeightMap, WNaiveDate},
    utils::timestamp_to_naive_date,
};

use super::ProcessedBlockData;

pub struct BlockMetadataDataset {
    pub date: HeightMap<WNaiveDate>,
    pub timestamp: HeightMap<u32>,
}

impl BlockMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/block_metadata");
        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            date: HeightMap::new_on_disk_bin(&f("date")),
            timestamp: HeightMap::new_on_disk_bin(&f("timestamp")),
        })
    }
}

impl AnyDataset for BlockMetadataDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height, timestamp, ..
        }: &ProcessedBlockData,
    ) {
        self.timestamp.insert(height, timestamp);

        self.date
            .insert(height, WNaiveDate::wrap(timestamp_to_naive_date(timestamp)));
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.date, &self.timestamp]
    }
}
