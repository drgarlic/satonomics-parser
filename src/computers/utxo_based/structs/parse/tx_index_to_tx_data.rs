use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bincode::{Decode, Encode};

use crate::utils::Snapshot;

use super::TxData;

#[derive(Encode, Decode, Default)]
/// `tx_index` => `txout_index` at `vout` 0
pub struct TxIndexToTxData(BTreeMap<u32, TxData>);

impl Snapshot for TxIndexToTxData {
    fn name<'a>() -> &'a str {
        "height_to_aged__tx_index_to_tx_data"
    }
}

impl Deref for TxIndexToTxData {
    type Target = BTreeMap<u32, TxData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxIndexToTxData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
