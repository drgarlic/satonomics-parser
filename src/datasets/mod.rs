use std::{collections::BTreeMap, thread};

use chrono::NaiveDate;
use itertools::Itertools;

mod _traits;
mod address;
mod block_metadata;
mod coinblocks;
mod coindays;
mod date_metadata;
mod price;
mod rewards;
mod subs;
mod transaction_metadata;
mod utxo;

pub use _traits::*;
use address::*;
use block_metadata::*;
use coinblocks::*;
use coindays::*;
use date_metadata::*;
use parking_lot::{lock_api::MutexGuard, RawMutex};
use price::*;
use rewards::*;
pub use subs::*;
use transaction_metadata::*;
use utxo::*;

use crate::{
    databases::Databases,
    io::Json,
    parse::{AddressData, AddressRealizedData, BlockData, BlockPath},
    states::{SplitPricePaidStates, SplitRealizedStates, SplitUnrealizedStates, States},
};

pub struct ProcessedDateData {
    pub block_count: usize,
    pub first_height: usize,
    pub height: usize,
    pub date: NaiveDate,
}

pub struct SortedBlockData<'a> {
    pub reversed_date_index: u16,
    pub year: u16,
    pub block_data: &'a BlockData,
}

pub struct ProcessedBlockData<'a> {
    pub address_index_to_address_realized_data: &'a BTreeMap<u32, AddressRealizedData>,
    pub address_index_to_removed_address_data: &'a BTreeMap<u32, AddressData>,
    pub block_path_to_spent_value: &'a BTreeMap<BlockPath, u64>,
    pub block_price: f32,
    pub coinbase: u64,
    pub coinbase_vec: &'a Vec<u64>,
    pub coinblocks_destroyed_vec: &'a Vec<f64>,
    pub coindays_destroyed_vec: &'a Vec<f64>,
    pub databases: &'a Databases,
    pub date: NaiveDate,
    pub date_price: f32,
    pub fees: &'a Vec<u64>,
    pub fees_vec: &'a Vec<Vec<u64>>,
    pub first_date_height: usize,
    pub height: usize,
    pub is_date_last_block: bool,
    pub sats_sent: u64,
    pub sats_sent_vec: &'a Vec<u64>,
    pub sorted_block_data_vec: Option<Vec<SortedBlockData<'a>>>,
    pub split_price_paid_states: &'a Option<SplitPricePaidStates>,
    pub split_realized_states: &'a mut Option<SplitRealizedStates>,
    pub split_unrealized_states_date: &'a Option<SplitUnrealizedStates>,
    pub split_unrealized_states_height: &'a Option<SplitUnrealizedStates>,
    pub states: &'a States,
    pub subsidy: u64,
    pub subsidy_vec: &'a Vec<u64>,
    pub subsidy_in_dollars: f32,
    pub subsidy_in_dollars_vec: &'a Vec<f32>,
    pub timestamp: u32,
    pub transaction_count: usize,
    pub transaction_count_vec: &'a Vec<usize>,
}

pub struct AllDatasets {
    pub address: AddressDatasets,
    pub price: PriceDatasets,
    pub utxo: UTXODatasets,

    block_metadata: BlockMetadataDataset,
    coinblocks: CoinblocksDataset,
    coindays: CoindaysDataset,
    date_metadata: DateMetadataDataset,
    rewards: RewardsDataset,
    transaction_metadata: TransactionMetadataDataset,
}

impl AllDatasets {
    pub fn import() -> color_eyre::Result<Self> {
        let path = "./datasets";

        thread::scope(|scope| {
            let date_metadata_handle = scope.spawn(|| DateMetadataDataset::import(path));

            let coinblocks_handle = scope.spawn(|| CoinblocksDataset::import(path));

            let coindays_handle = scope.spawn(|| CoindaysDataset::import(path));

            let rewards_handle = scope.spawn(|| RewardsDataset::import(path));

            let block_metadata_handle = scope.spawn(|| BlockMetadataDataset::import(path));

            let utxo_handle = scope.spawn(|| UTXODatasets::import(path));

            let transaction_metadata = TransactionMetadataDataset::import(path)?;

            let address = AddressDatasets::import(path)?;

            let price_handle = PriceDatasets::import()?;

            let this = Self {
                address,
                block_metadata: block_metadata_handle.join().unwrap()?,
                coinblocks: coinblocks_handle.join().unwrap()?,
                coindays: coindays_handle.join().unwrap()?,
                date_metadata: date_metadata_handle.join().unwrap()?,
                price: price_handle,
                rewards: rewards_handle.join().unwrap()?,
                utxo: utxo_handle.join().unwrap()?,
                transaction_metadata,
            };

            this.export_path_to_type()?;

            Ok(this)
        })
    }

    pub fn get_date_to_last_height(&self) -> MutexGuard<'_, RawMutex, BTreeMap<String, usize>> {
        self.date_metadata.last_height.unsafe_inner()
    }

    pub fn export_path_to_type(&self) -> color_eyre::Result<()> {
        let path_to_type: BTreeMap<&str, &str> = self
            .to_any_dataset_vec()
            .iter()
            .flat_map(|dataset| {
                vec![
                    dataset
                        .to_any_date_map_vec()
                        .iter()
                        .map(|map| (map.path(), map.t_name()))
                        .collect_vec(),
                    dataset
                        .to_any_height_map_vec()
                        .iter()
                        .map(|map| (map.path(), map.t_name()))
                        .collect_vec(),
                ]
            })
            .flatten()
            .collect();

        Json::export("./datasets/paths.json", &path_to_type)
    }
}

impl AnyDatasets for AllDatasets {
    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        vec![
            self.address.to_any_dataset_vec(),
            self.price.to_any_dataset_vec(),
            self.utxo.to_any_dataset_vec(),
            vec![
                &self.block_metadata,
                &self.coinblocks,
                &self.coindays,
                &self.date_metadata,
                &self.rewards,
                &self.transaction_metadata,
            ],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    fn name<'a>() -> &'a str {
        "datasets"
    }
}
