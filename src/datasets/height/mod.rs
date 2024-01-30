use std::collections::BTreeMap;
use std::thread;

use chrono::NaiveDate;

use crate::{
    states::States,
    structs::{AddressData, AddressRealizedData, BlockPath},
};

mod _trait;
// mod address;
mod coinblocks;
mod coindays;
mod rewards;
mod subs;
mod time;
mod utxo;

pub use _trait::*;
// use address::*;
use coinblocks::*;
use coindays::*;
use rewards::*;
use subs::*;
use time::*;
use utxo::*;

pub struct ProcessedBlockData<'a> {
    pub address_index_to_address_realized_data: &'a BTreeMap<u32, AddressRealizedData>,
    pub address_index_to_removed_address_data: &'a BTreeMap<u32, AddressData>,
    pub block_path_to_spent_value: &'a BTreeMap<BlockPath, u64>,
    pub coinbase: u64,
    pub coinblocks_destroyed: f64,
    pub coindays_destroyed: f64,
    pub date: NaiveDate,
    pub date_price: f32,
    pub states: &'a States,
    pub fees: u64,
    pub height: usize,
    pub is_date_last_block: bool,
    pub block_price: f32,
    pub timestamp: u32,
}

pub struct HeightDatasets {
    // address: AddressDatasets,
    coinblocks: CoinblocksDataset,
    coindays: CoindaysDataset,
    rewards: RewardsDataset,
    time: TimeDataset,
    utxo: UTXODatasets,
}

impl HeightDatasets {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let path = format!("{parent_path}/height");

        thread::scope(|scope| {
            // let address_handle = scope.spawn(|| AddressDatasets::import(&path));
            let coinblocks_handle = scope.spawn(|| CoinblocksDataset::import(&path));
            let coindays_handle = scope.spawn(|| CoindaysDataset::import(&path));
            let rewards_handle = scope.spawn(|| RewardsDataset::import(&path));
            let time_handle = scope.spawn(|| TimeDataset::import(&path));
            let utxo_handle = scope.spawn(|| UTXODatasets::import(&path));

            Ok(Self {
                // address: address_handle.join().unwrap()?,
                coinblocks: coinblocks_handle.join().unwrap()?,
                coindays: coindays_handle.join().unwrap()?,
                rewards: rewards_handle.join().unwrap()?,
                time: time_handle.join().unwrap()?,
                utxo: utxo_handle.join().unwrap()?,
            })
        })
    }
}

impl AnyHeightDatasets for HeightDatasets {
    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightDataset + Send + Sync)> {
        let flat_datasets: Vec<&(dyn AnyHeightDataset + Send + Sync)> =
            vec![&self.rewards, &self.coinblocks, &self.coindays, &self.time];

        [
            flat_datasets,
            // self.address.to_vec(),
            self.utxo.to_any_height_map_vec(),
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }
}
