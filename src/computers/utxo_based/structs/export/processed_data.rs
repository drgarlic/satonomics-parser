use std::collections::BTreeMap;

use crate::computers::utxo_based::{structs::AddressIndexToAddressData, BlockPath, DateDataVec};

pub struct ProcessedData<'a> {
    pub address_index_to_address_data: &'a AddressIndexToAddressData,
    pub address_index_to_spent_value: &'a BTreeMap<u32, f64>,
    pub block_path_to_spent_value: &'a BTreeMap<BlockPath, f64>,
    pub coinbase: f64,
    pub coinblocks_destroyed: f64,
    pub coindays_destroyed: f64,
    pub date_data_vec: &'a DateDataVec,
    pub fees: f64,
    pub height: usize,
    pub price: f32,
    pub timestamp: u32,
}
