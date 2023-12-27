use std::collections::BTreeMap;

use crate::computers::utxo_based::{structs::AddressIndexToAddressData, BlockPath, DateDataVec};

pub struct DatasetInsertData<'a> {
    pub address_index_to_address_data: &'a AddressIndexToAddressData,
    pub date_data_vec: &'a DateDataVec,
    pub height: usize,
    pub price: f32,
    pub coinbase: f64,
    pub fees: f64,
    pub coinblocks_destroyed: f64,
    pub coindays_destroyed: f64,
    pub block_path_to_spent_value: &'a BTreeMap<BlockPath, f64>,
    pub address_index_to_spent_value: &'a BTreeMap<u32, f64>,
}
