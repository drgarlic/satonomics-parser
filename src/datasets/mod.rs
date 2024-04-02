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
use rayon::prelude::*;
pub use subs::*;
use transaction::*;
use utxo::*;

use crate::{
    actions::{ReceivedData, SpentData},
    databases::Databases,
    io::Json,
    parse::{
        AddressData, AddressRealizedData, BiMap, BlockData, BlockPath, DateMap, HeightMap,
        HeightToDateConverter,
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
    pub databases: &'a Databases,
    pub date: NaiveDate,
    pub date_price: f32,
    pub fees: &'a Vec<u64>,
    pub first_date_height: usize,
    pub height: usize,
    pub is_date_last_block: bool,
    pub satblocks_destroyed: u64,
    pub satdays_destroyed: u64,
    pub sats_sent: u64,
    pub sorted_block_data_vec: Option<Vec<SortedBlockData<'a>>>,
    pub split_input_states: &'a mut Option<SplitInputStates>,
    pub split_output_states: &'a mut Option<SplitOutputStates>,
    pub split_price_paid_states: &'a Option<SplitPricePaidStates>,
    pub split_realized_states: &'a mut Option<SplitRealizedStates>,
    pub split_unrealized_states_date: &'a Option<SplitUnrealizedStates>,
    pub split_unrealized_states_height: &'a Option<SplitUnrealizedStates>,
    pub states: &'a States,
    pub timestamp: u32,
    pub transaction_count: usize,
}

pub struct ExportData<'a> {
    pub annualized_transaction_volume: &'a BiMap<f32>,
    pub circulating_supply: &'a BiMap<f32>,
    pub convert_last_height_to_date: &'a HeightToDateConverter<'a>,
    pub convert_sum_heights_to_date: &'a HeightToDateConverter<'a>,
    pub yearly_inflation_rate: &'a BiMap<f32>,
    pub height_price: &'a HeightMap<f32>,
    pub date_price: &'a DateMap<f32>,
    pub realized_cap: &'a BiMap<f32>,
    pub realized_price: &'a BiMap<f32>,
    pub subsidy_in_dollars: &'a BiMap<f32>,
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
                dataset
                    .to_any_exported_map_vec()
                    .into_iter()
                    .flat_map(|map| map.exported_paths_with_t_name())
                    .chain(
                        dataset
                            .to_any_exported_map_vec()
                            .into_iter()
                            .flat_map(|map| map.exported_paths_with_t_name()),
                    )
                    .collect_vec()
            })
            .collect();

        Json::export("./datasets/paths.json", &path_to_type)
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self._export_if_needed(None, true)
    }

    pub fn export_if_needed(
        &self,
        date: NaiveDate,
        height: usize,
        compute: bool,
    ) -> color_eyre::Result<()> {
        self._export_if_needed(Some((height, date)), compute)
    }

    pub fn _export_if_needed(
        &self,
        height_and_date: Option<(usize, NaiveDate)>,
        compute: bool,
    ) -> color_eyre::Result<()> {
        let export_data = ExportData {
            // They all need to be:
            // - Be stored memory
            // - Either inserted or computed in the prepare function
            annualized_transaction_volume: &self.transaction.annualized_volume,
            circulating_supply: &self.utxo.all.subs.supply.total,
            yearly_inflation_rate: &self.mining.yearly_inflation_rate,
            height_price: &self.price.height.closes,
            date_price: &self.price.date.closes,
            realized_cap: &self.utxo.all.subs.price_paid.realized_cap,
            realized_price: &self.utxo.all.subs.price_paid.realized_price,
            subsidy_in_dollars: &self.mining.subsidy_in_dollars,

            convert_last_height_to_date: &HeightToDateConverter::Last(
                &self.date_metadata.first_height,
            ),
            convert_sum_heights_to_date: &HeightToDateConverter::Sum {
                first_height: &self.date_metadata.first_height,
                last_height: &self.date_metadata.last_height,
            },
        };

        let vec = self.to_generic_dataset_vec();

        vec.iter().for_each(|dataset| dataset.prepare(&export_data));

        vec.into_par_iter()
            .filter(|dataset| {
                if let Some((height, date)) = height_and_date {
                    dataset.should_insert(height, date)
                } else {
                    true
                }
            })
            .try_for_each(|dataset| -> color_eyre::Result<()> {
                dataset.import_tmp_data();

                if compute {
                    dataset.compute(&export_data);
                }

                dataset.export_then_clean()
            })?;

        Ok(())
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
