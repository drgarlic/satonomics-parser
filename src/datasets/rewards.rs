use chrono::NaiveDate;
use itertools::Itertools;

use crate::{
    bitcoin::sats_to_btc,
    datasets::AnyDataset,
    parse::{AnyHeightMap, BiMap, DateMap},
};

use super::ProcessedBlockData;

pub struct RewardsDataset {
    name: &'static str,
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,

    pub fees_sumed: BiMap<f64>,
    pub subsidy: BiMap<f64>,
    pub coinbase: BiMap<f64>,
    pub subsidy_in_dollars: BiMap<f32>,
    pub last_subsidy: DateMap<f64>,
}

impl RewardsDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let name = "rewards";

        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            name,
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,
            fees_sumed: BiMap::new_on_disk_bin(&f("fees/sumed")),
            coinbase: BiMap::new_on_disk_bin(&f("coinbase")),
            subsidy: BiMap::new_on_disk_bin(&f("subsidy")),
            subsidy_in_dollars: BiMap::new_on_disk_bin(&f("subsidy_in_dollars")),
            last_subsidy: DateMap::new_on_disk_bin(&f("last_subsidy")),
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
    }
}

impl AnyDataset for RewardsDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height,
            date,
            coinbase,
            coinbase_vec,
            fees,
            fees_vec,
            subsidy,
            subsidy_vec,
            subsidy_in_dollars,
            subsidy_in_dollars_vec,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
    ) {
        self.coinbase.height.insert(height, sats_to_btc(coinbase));

        let fees_sumed = fees.iter().sum();

        self.fees_sumed
            .height
            .insert(height, sats_to_btc(fees_sumed));

        let subsidy_in_btc = sats_to_btc(subsidy);

        self.subsidy.height.insert(height, subsidy_in_btc);

        self.subsidy_in_dollars
            .height
            .insert(height, subsidy_in_dollars);

        if is_date_last_block {
            self.coinbase
                .date
                .insert(date, sats_to_btc(coinbase_vec.iter().sum()));

            self.last_subsidy.insert(date, subsidy_in_btc);

            let fees_sumed = fees_vec
                .iter()
                .map(|vec| vec.iter().sum::<u64>())
                .collect_vec();

            self.fees_sumed
                .date
                .insert(date, sats_to_btc(fees_sumed.iter().sum()));

            self.subsidy
                .date
                .insert(date, sats_to_btc(subsidy_vec.iter().sum()));

            self.subsidy_in_dollars
                .date
                .insert(date, subsidy_in_dollars_vec.iter().sum());
        }
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.fees_sumed.height,
            &self.subsidy.height,
            &self.subsidy_in_dollars.height,
        ]
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn crate::parse::AnyDateMap + Send + Sync)> {
        vec![
            &self.fees_sumed.date,
            &self.subsidy.date,
            &self.last_subsidy,
            &self.subsidy_in_dollars.date,
        ]
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
