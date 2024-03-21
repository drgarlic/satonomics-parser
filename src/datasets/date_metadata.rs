use chrono::NaiveDate;

use crate::{
    datasets::AnyDataset,
    parse::{AnyDateMap, DateMap},
};

use super::ProcessedDateData;

pub struct DateMetadataDataset {
    name: &'static str,
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,
    pub first_height: DateMap<usize>,
    pub last_height: DateMap<usize>,
    pub block_count: DateMap<usize>,
}

impl DateMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let name = "date_metadata";

        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            name,
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,
            first_height: DateMap::new_on_disk_bin(&f("first_height")),
            last_height: DateMap::new_in_memory_bin(&f("last_height")),
            block_count: DateMap::new_on_disk_bin(&f("block_count")),
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
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

    fn name(&self) -> &str {
        self.name
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }
}
