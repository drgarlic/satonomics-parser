use chrono::NaiveDate;

use crate::{
    structs::{AnyHeightMap, HeightMap},
    traits::HeightDataset,
    utils::timestamp_to_naive_date,
};

use super::ProcessedData;

pub struct BlockMetadataDataset {
    pub height_to_date: HeightMap<NaiveDate>,
    pub height_to_timestamp: HeightMap<u32>,
}

impl BlockMetadataDataset {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self {
            height_to_date: HeightMap::new("height_to_date.json"),
            height_to_timestamp: HeightMap::new("height_to_timestamp.json"),
        })
    }
}

impl<'a> HeightDataset<ProcessedData<'a>> for BlockMetadataDataset {
    fn insert(&self, processed_data: &ProcessedData) {
        let &ProcessedData {
            height, timestamp, ..
        } = processed_data;

        self.height_to_timestamp.insert(height, timestamp);

        self.height_to_date
            .insert(height, timestamp_to_naive_date(timestamp));
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.height_to_date, &self.height_to_timestamp]
    }
}
