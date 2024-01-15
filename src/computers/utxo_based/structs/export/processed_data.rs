use std::collections::BTreeMap;

use chrono::NaiveDate;
use ordered_float::OrderedFloat;

use crate::computers::utxo_based::{structs::AddressIndexToAddressData, BlockPath, DateDataVec};

pub struct ProcessedData<'a> {
    pub address_index_to_address_data: &'a AddressIndexToAddressData,
    pub address_index_to_spent_value: &'a BTreeMap<u32, BTreeMap<OrderedFloat<f32>, u64>>,
    pub block_path_to_spent_value: &'a BTreeMap<BlockPath, u64>,
    pub coinbase: u64,
    pub coinblocks_destroyed: f64,
    pub coindays_destroyed: f64,
    pub date: NaiveDate,
    pub date_data_vec: &'a DateDataVec,
    pub fees: u64,
    pub height: usize,
    pub price: f32,
    pub timestamp: u32,
}
