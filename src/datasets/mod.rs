use std::{collections::BTreeMap, thread};

use chrono::NaiveDate;
use itertools::Itertools;

mod _traits;
mod address;
mod block_metadata;
mod coindays;
mod cointime;
mod date_metadata;
mod mining;
mod price;
mod subs;
mod transaction;
mod utxo;

pub use _traits::*;
use address::*;
use block_metadata::*;
use coindays::*;
use cointime::*;
use date_metadata::*;
use mining::*;
use price::*;
pub use subs::*;
use transaction::*;
use utxo::*;

use crate::{
    actions::{ReceivedData, SpentData},
    databases::Databases,
    io::Json,
    parse::{
        AddressData, AddressRealizedData, BiMap, BlockData, BlockPath, HeightToDateConverter,
        WNaiveDate,
    },
    states::{
        SplitInputStates, SplitOutputStates, SplitPricePaidStates, SplitRealizedStates,
        SplitUnrealizedStates, States,
    },
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
    pub block_path_to_received_data: &'a BTreeMap<BlockPath, ReceivedData>,
    pub block_path_to_spent_data: &'a BTreeMap<BlockPath, SpentData>,
    pub block_price: f32,
    pub coinbase: u64,
    pub coinbase_vec: &'a Vec<u64>,
    pub databases: &'a Databases,
    pub date: NaiveDate,
    pub date_price: f32,
    pub fees: &'a Vec<u64>,
    pub fees_vec: &'a Vec<Vec<u64>>,
    pub first_date_height: usize,
    pub height: usize,
    pub is_date_last_block: bool,
    pub satblocks_destroyed: u64,
    pub satblocks_destroyed_vec: &'a Vec<u64>,
    pub satdays_destroyed: u64,
    pub satdays_destroyed_vec: &'a Vec<u64>,
    pub sats_sent: u64,
    pub sats_sent_vec: &'a Vec<u64>,
    pub sorted_block_data_vec: Option<Vec<SortedBlockData<'a>>>,
    pub split_input_states: &'a mut Option<SplitInputStates>,
    pub split_output_states: &'a mut Option<SplitOutputStates>,
    pub split_price_paid_states: &'a Option<SplitPricePaidStates>,
    pub split_realized_states: &'a mut Option<SplitRealizedStates>,
    pub split_unrealized_states_date: &'a Option<SplitUnrealizedStates>,
    pub split_unrealized_states_height: &'a Option<SplitUnrealizedStates>,
    pub states: &'a States,
    pub subsidy: u64,
    pub subsidy_in_dollars: f32,
    pub subsidy_in_dollars_vec: &'a Vec<f32>,
    pub subsidy_vec: &'a Vec<u64>,
    pub timestamp: u32,
    pub transaction_count: usize,
    pub transaction_count_vec: &'a Vec<usize>,
}

pub struct ExportData<'a> {
    // pub height: usize,
    pub annualized_transaction_volume: &'a BiMap<f32>,
    pub circulating_supply: &'a BiMap<f32>,
    pub date_to_first_height: &'a BTreeMap<WNaiveDate, usize>,
    pub date_to_last_height: &'a BTreeMap<WNaiveDate, usize>,
    pub last_height_to_date: &'a HeightToDateConverter<'a>,
    pub sum_heights_to_date: &'a HeightToDateConverter<'a>,
    pub inflation_rate: &'a BiMap<f32>,
    pub price: &'a BiMap<f32>,
    pub realized_cap: &'a BiMap<f32>,
    pub realized_price: &'a BiMap<f32>,
    pub subsidy_in_dollars: &'a BiMap<f32>,
}

pub struct AllDatasets {
    min_initial_state: MinInitialState,

    pub address: AddressDatasets,
    pub price: PriceDatasets,
    pub utxo: UTXODatasets,

    block_metadata: BlockMetadataDataset,
    cointime: CointimeDataset,
    coindays: CoindaysDataset,
    pub date_metadata: DateMetadataDataset,
    mining: MiningDataset,
    transaction: TransactionDataset,
}

impl AllDatasets {
    pub fn import() -> color_eyre::Result<Self> {
        let path = "./datasets";

        thread::scope(|scope| {
            let date_metadata_handle = scope.spawn(|| DateMetadataDataset::import(path));

            let cointime_handle = scope.spawn(|| CointimeDataset::import(path));

            let coindays_handle = scope.spawn(|| CoindaysDataset::import(path));

            let mining_handle = scope.spawn(|| MiningDataset::import(path));

            let block_metadata_handle = scope.spawn(|| BlockMetadataDataset::import(path));

            let utxo_handle = scope.spawn(|| UTXODatasets::import(path));

            let transaction_handle = scope.spawn(|| TransactionDataset::import(path));

            let address = AddressDatasets::import(path)?;

            let price_handle = PriceDatasets::import()?;

            let s = Self {
                min_initial_state: MinInitialState::default(),

                address,
                block_metadata: block_metadata_handle.join().unwrap()?,
                cointime: cointime_handle.join().unwrap()?,
                coindays: coindays_handle.join().unwrap()?,
                date_metadata: date_metadata_handle.join().unwrap()?,
                price: price_handle,
                mining: mining_handle.join().unwrap()?,
                utxo: utxo_handle.join().unwrap()?,
                transaction: transaction_handle.join().unwrap()?,
            };

            s.min_initial_state.compute_from_datasets(&s);

            s.export_path_to_type()?;

            Ok(s)
        })
    }

    pub fn export_path_to_type(&self) -> color_eyre::Result<()> {
        let path_to_type: BTreeMap<&str, &str> = self
            .to_generic_dataset_vec()
            .iter()
            .flat_map(|dataset| {
                vec![
                    dataset
                        .to_any_inserted_date_map_vec()
                        .iter()
                        .map(|map| (map.path(), map.t_name()))
                        .collect_vec(),
                    dataset
                        .to_any_inserted_height_map_vec()
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
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_generic_dataset_vec(&self) -> Vec<&(dyn GenericDataset + Send + Sync)> {
        vec![
            self.address.to_generic_dataset_vec(),
            self.price.to_generic_dataset_vec(),
            self.utxo.to_generic_dataset_vec(),
            vec![
                &self.block_metadata,
                &self.cointime,
                &self.coindays,
                &self.date_metadata,
                &self.mining,
                &self.transaction,
            ],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}
