use std::{collections::BTreeMap, sync::RwLockReadGuard, thread};

use chrono::NaiveDate;

mod _traits;
// mod address;
mod block_metadata;
mod coinblocks;
mod coindays;
mod date_metadata;
mod price;
mod rewards;
mod subs;
mod utxo;

pub use _traits::*;
// use address::*;
use block_metadata::*;
use coinblocks::*;
use coindays::*;
use date_metadata::*;
use itertools::Itertools;
use price::*;
use rewards::*;
use subs::*;
use utxo::*;

use crate::{
    states::States,
    structs::{AddressData, AddressRealizedData, BlockPath},
};

pub struct ProcessedDateData {
    pub block_count: usize,
    pub first_height: usize,
    pub height: usize,
    pub date: NaiveDate,
}

pub struct ProcessedBlockData<'a> {
    pub address_index_to_address_realized_data: &'a BTreeMap<u32, AddressRealizedData>,
    pub address_index_to_removed_address_data: &'a BTreeMap<u32, AddressData>,
    pub block_path_to_spent_value: &'a BTreeMap<BlockPath, u64>,
    pub coinbase_vec: &'a Vec<u64>,
    pub coinblocks_destroyed_vec: &'a Vec<f64>,
    pub coindays_destroyed_vec: &'a Vec<f64>,
    pub date: NaiveDate,
    pub date_price: f32,
    pub states: &'a States,
    pub fees_vec: &'a Vec<Vec<u64>>,
    pub height: usize,
    pub is_date_last_block: bool,
    pub block_price: f32,
    pub timestamp: u32,
}

pub struct AllDatasets {
    // address: AddressDatasets,
    coinblocks: CoinblocksDataset,
    coindays: CoindaysDataset,
    rewards: RewardsDataset,
    block_metadata: BlockMetadataDataset,
    utxo: UTXODatasets,
    date_metadata: DateMetadataDataset,
    pub price: PriceDatasets,
}

impl AllDatasets {
    pub fn import() -> color_eyre::Result<Self> {
        let path = "./datasets";

        thread::scope(|scope| {
            let date_metadata_handle = scope.spawn(|| DateMetadataDataset::import(path));

            // let address_handle = scope.spawn(|| AddressDatasets::import(&path));
            //
            let coinblocks_handle = scope.spawn(|| CoinblocksDataset::import(path));

            let coindays_handle = scope.spawn(|| CoindaysDataset::import(path));

            let rewards_handle = scope.spawn(|| RewardsDataset::import(path));

            let block_metadata_handle = scope.spawn(|| BlockMetadataDataset::import(path));

            let utxo_handle = scope.spawn(|| UTXODatasets::import(path));

            Ok(Self {
                date_metadata: date_metadata_handle.join().unwrap()?,
                price: PriceDatasets::import(path)?,
                // address: address_handle.join().unwrap()?,
                coinblocks: coinblocks_handle.join().unwrap()?,
                coindays: coindays_handle.join().unwrap()?,
                rewards: rewards_handle.join().unwrap()?,
                block_metadata: block_metadata_handle.join().unwrap()?,
                utxo: utxo_handle.join().unwrap()?,
            })
        })
    }

    pub fn get_date_to_last_height(&self) -> RwLockReadGuard<'_, BTreeMap<String, usize>> {
        self.date_metadata.last_height.unsafe_inner()
    }
}

impl AnyDatasets for AllDatasets {
    fn to_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        let vec: Vec<&(dyn AnyDataset + Send + Sync)> = vec![
            &self.date_metadata,
            &self.rewards,
            &self.coinblocks,
            &self.coindays,
            &self.block_metadata,
        ];

        vec![
            vec,
            // self.address.to_vec(),
            self.price.to_vec(),
            self.utxo.to_vec(),
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}
