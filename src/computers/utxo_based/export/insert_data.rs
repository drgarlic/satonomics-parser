use std::collections::BTreeMap;

use crate::computers::utxo_based::{BlockPath, DateDataVec};

pub struct DatasetInsertData<'a> {
    pub date_data_vec: &'a DateDataVec,
    pub height: usize,
    pub price: f32,
    pub coinbase: f64,
    pub fees: f64,
    pub stxos: &'a BTreeMap<BlockPath, f64>,
    pub coinblocks_destroyed: f64,
}
