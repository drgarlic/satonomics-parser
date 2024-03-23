use chrono::NaiveDate;

use crate::{
    bitcoin::sats_to_btc,
    datasets::AnyDataset,
    parse::{AnyDateMap, AnyHeightMap, BiMap},
};

use super::ProcessedBlockData;

pub struct CoinblocksDataset {
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,

    pub destroyed: BiMap<f32>,
}

impl CoinblocksDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let name = "coinblocks";

        let f = |s: &str| format!("{parent_path}/{name}/{s}");

        let mut s = Self {
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,
            destroyed: BiMap::new_on_disk_bin(&f("destroyed")),
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
    }
}

impl AnyDataset for CoinblocksDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height,
            satblocks_destroyed,
            satblocks_destroyed_vec,
            is_date_last_block,
            date,
            ..
        }: &ProcessedBlockData,
    ) {
        self.destroyed
            .height
            .insert(height, sats_to_btc(satblocks_destroyed));

        if is_date_last_block {
            self.destroyed
                .date
                .insert(date, sats_to_btc(satblocks_destroyed_vec.iter().sum()))
        }
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.destroyed.height]
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.destroyed.date]
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }
}
