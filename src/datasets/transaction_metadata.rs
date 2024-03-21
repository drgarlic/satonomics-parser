use chrono::NaiveDate;

use crate::{
    bitcoin::sats_to_btc,
    datasets::AnyDataset,
    parse::{AnyDateMap, AnyHeightMap, BiMap},
};

use super::ProcessedBlockData;

pub struct TransactionMetadataDataset {
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,

    pub count: BiMap<usize>,
    pub volume: BiMap<f64>,
}

impl TransactionMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/transaction/{s}");

        let mut s = Self {
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,

            count: BiMap::new_on_disk_bin(&f("count")),
            volume: BiMap::new_on_disk_bin(&f("volume")),
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
    }
}

impl AnyDataset for TransactionMetadataDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height,
            date,
            is_date_last_block,
            sats_sent,
            sats_sent_vec,
            transaction_count,
            transaction_count_vec,
            ..
        }: &ProcessedBlockData,
    ) {
        self.count.height.insert(height, transaction_count);

        self.volume.height.insert(height, sats_to_btc(sats_sent));

        if is_date_last_block {
            self.count
                .date
                .insert(date, transaction_count_vec.iter().sum());

            self.volume
                .date
                .insert(date, sats_to_btc(sats_sent_vec.iter().sum()));
        }
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.count.date, &self.volume.date]
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count.height, &self.volume.height]
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }
}
