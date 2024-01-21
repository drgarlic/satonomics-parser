use std::collections::BTreeMap;
use std::thread;

use chrono::NaiveDate;

use crate::computers::utxo_based::{AddressRealizedData, BlockPath, States};

mod _trait;
mod aged;
mod coinblocks;
mod coindays;
mod entity;
mod price;
mod rewards;
mod time;

pub use _trait::*;
use aged::*;
use coinblocks::*;
use coindays::*;
use entity::*;
use price::*;
use rewards::*;
use time::*;

pub struct ProcessedData<'a> {
    pub address_index_to_address_realized_data: &'a BTreeMap<u32, AddressRealizedData>,
    pub block_path_to_spent_value: &'a BTreeMap<BlockPath, u64>,
    pub coinbase: u64,
    pub coinblocks_destroyed: f64,
    pub coindays_destroyed: f64,
    pub date: NaiveDate,
    pub states: &'a States,
    pub fees: u64,
    pub height: usize,
    pub price: f32,
    pub timestamp: u32,
}

pub struct HeightDatasets {
    height_to_aged: AgedDatasets,

    height_to_entity: EntityDatasets,

    height_to_rewards: RewardsDataset,

    height_to_coinblocks: CoinblocksDataset,

    height_to_coindays: CoindaysDataset,

    height_to_time: TimeDataset,
}

const HEIGHT_DATASETS_PATH: &str = "./datasets/block";

impl HeightDatasets {
    pub fn import() -> color_eyre::Result<Self> {
        let path = HEIGHT_DATASETS_PATH;

        let height_to_aged_handle = thread::spawn(|| AgedDatasets::import(path));

        let height_to_entity_handle = thread::spawn(|| EntityDatasets::import(path));

        let height_to_rewards_handle = thread::spawn(|| RewardsDataset::import(path));

        let height_to_coinblocks_handle = thread::spawn(|| CoinblocksDataset::import(path));

        let height_to_coindays_handle = thread::spawn(|| CoindaysDataset::import(path));

        let height_to_time_handle = thread::spawn(|| TimeDataset::import(path));

        Ok(Self {
            height_to_aged: height_to_aged_handle.join().unwrap()?,

            height_to_entity: height_to_entity_handle.join().unwrap()?,

            height_to_rewards: height_to_rewards_handle.join().unwrap()?,

            height_to_coinblocks: height_to_coinblocks_handle.join().unwrap()?,

            height_to_coindays: height_to_coindays_handle.join().unwrap()?,

            height_to_time: height_to_time_handle.join().unwrap()?,
        })
    }
}

impl HeightDatasetsTrait for HeightDatasets {
    fn to_vec(&self) -> Vec<&(dyn HeightDatasetTrait + Send + Sync)> {
        let flat_datasets: Vec<&(dyn HeightDatasetTrait + Send + Sync)> = vec![
            &self.height_to_rewards,
            &self.height_to_coinblocks,
            &self.height_to_coindays,
            &self.height_to_time,
        ];

        [
            flat_datasets,
            self.height_to_aged.to_vec(),
            self.height_to_entity.to_vec(),
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }
}
