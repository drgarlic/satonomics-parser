use std::{collections::BTreeMap, ops::RangeInclusive, thread};

use chrono::NaiveDate;
use itertools::Itertools;
use rayon::prelude::*;

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
pub use address::*;
pub use block_metadata::*;
pub use coindays::*;
pub use cointime::*;
pub use date_metadata::*;
pub use mining::*;
pub use price::*;
pub use subs::*;
pub use transaction::*;
pub use utxo::*;

use crate::{
    databases::Databases,
    io::Json,
    parse::{AddressData, AddressRealizedData},
    states::{
        AddressCohortsInputStates, AddressCohortsOneShotStates, AddressCohortsOutputStates,
        AddressCohortsRealizedStates, States, UTXOCohortsOneShotStates, UTXOCohortsReceivedStates,
        UTXOCohortsSentStates,
    },
};

pub struct ProcessedBlockData<'a> {
    pub address_cohorts_input_states: &'a Option<AddressCohortsInputStates>,
    pub address_cohorts_one_shot_states: &'a Option<AddressCohortsOneShotStates>,
    pub address_cohorts_output_states: &'a Option<AddressCohortsOutputStates>,
    pub address_cohorts_realized_states: &'a Option<AddressCohortsRealizedStates>,
    pub address_index_to_address_realized_data: &'a BTreeMap<u32, AddressRealizedData>,
    pub address_index_to_removed_address_data: &'a BTreeMap<u32, AddressData>,
    pub block_price: f32,
    pub coinbase: u64,
    pub databases: &'a Databases,
    pub date: NaiveDate,
    pub date_first_height: usize,
    pub date_blocks_range: &'a RangeInclusive<usize>,
    pub date_price: f32,
    pub fees: &'a Vec<u64>,
    pub height: usize,
    pub is_date_last_block: bool,
    pub satblocks_destroyed: u64,
    pub satdays_destroyed: u64,
    pub sats_sent: u64,
    pub states: &'a States,
    pub timestamp: u32,
    pub transaction_count: usize,
    pub utxo_cohorts_one_shot_states: &'a UTXOCohortsOneShotStates,
    pub utxo_cohorts_received_states: &'a UTXOCohortsReceivedStates,
    pub utxo_cohorts_sent_states: &'a UTXOCohortsSentStates,
}

pub struct AllDatasets {
    min_initial_state: MinInitialState,

    pub address: AddressDatasets,
    pub price: PriceDatasets,
    pub utxo: UTXODatasets,

    pub block_metadata: BlockMetadataDataset,
    pub cointime: CointimeDataset,
    pub coindays: CoindaysDataset,
    pub date_metadata: DateMetadataDataset,
    pub mining: MiningDataset,
    pub transaction: TransactionDataset,
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

            let transaction_handle = scope.spawn(|| TransactionDataset::import(path));

            let address = AddressDatasets::import(path)?;

            let utxo = UTXODatasets::import(path)?;

            let price = PriceDatasets::import()?;

            let block_metadata = block_metadata_handle.join().unwrap()?;

            let cointime = cointime_handle.join().unwrap()?;

            let coindays = coindays_handle.join().unwrap()?;

            let date_metadata = date_metadata_handle.join().unwrap()?;

            let mining = mining_handle.join().unwrap()?;

            let transaction = transaction_handle.join().unwrap()?;

            let mut s = Self {
                min_initial_state: MinInitialState::default(),

                address,
                block_metadata,
                cointime,
                coindays,
                date_metadata,
                price,
                mining,
                transaction,
                utxo,
            };

            s.min_initial_state
                .consume(MinInitialState::compute_from_datasets(&s));

            s.export_path_to_type()?;

            Ok(s)
        })
    }

    pub fn insert_data(&mut self, processed_block_data: ProcessedBlockData) {
        let ProcessedBlockData { height, date, .. } = processed_block_data;

        self.address.insert_data(&processed_block_data);

        self.utxo.insert_data(&processed_block_data);

        if self.block_metadata.should_insert(height, date) {
            self.block_metadata.insert_data(&processed_block_data);
        }

        if self.date_metadata.should_insert(height, date) {
            self.date_metadata.insert_data(&processed_block_data);
        }

        if self.coindays.should_insert(height, date) {
            self.coindays.insert_data(&processed_block_data);
        }

        if self.mining.should_insert(height, date) {
            self.mining
                .insert_data(&processed_block_data, &self.address);
        }

        if self.transaction.should_insert(height, date) {
            self.transaction
                .insert_data(&processed_block_data, &self.address);
        }

        if self.cointime.should_insert(height, date) {
            self.cointime.insert_data(
                &processed_block_data,
                &self.address,
                &self.mining,
                &self.transaction,
            );
        }
    }

    pub fn export_path_to_type(&self) -> color_eyre::Result<()> {
        let path_to_type: BTreeMap<&str, &str> = self
            .to_any_dataset_vec()
            .into_iter()
            .flat_map(|dataset| {
                dataset
                    .to_any_map_vec()
                    .into_iter()
                    .flat_map(|map| map.exported_path_with_t_name())
            })
            .collect();

        Json::export("./datasets/paths.json", &path_to_type)
    }

    pub fn export(&mut self) -> color_eyre::Result<()> {
        self.to_mut_any_dataset_vec()
            .into_iter()
            .for_each(|dataset| dataset.pre_export());

        self.to_any_dataset_vec()
            .into_par_iter()
            .try_for_each(|dataset| -> color_eyre::Result<()> { dataset.export() })?;

        self.to_mut_any_dataset_vec()
            .into_iter()
            .for_each(|dataset| dataset.post_export());

        Ok(())
    }
}

impl AnyDatasets for AllDatasets {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        vec![
            self.address.to_any_dataset_vec(),
            self.price.to_any_dataset_vec(),
            self.utxo.to_any_dataset_vec(),
            vec![
                &self.mining,
                &self.transaction,
                &self.block_metadata,
                &self.date_metadata,
                &self.cointime,
                &self.coindays,
            ],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    fn to_mut_any_dataset_vec(&mut self) -> Vec<&mut dyn AnyDataset> {
        vec![
            self.address.to_mut_any_dataset_vec(),
            self.price.to_mut_any_dataset_vec(),
            self.utxo.to_mut_any_dataset_vec(),
            vec![
                &mut self.mining,
                &mut self.transaction,
                &mut self.block_metadata,
                &mut self.date_metadata,
                &mut self.cointime,
                &mut self.coindays,
            ],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}
