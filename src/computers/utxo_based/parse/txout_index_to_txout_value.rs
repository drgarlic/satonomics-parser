use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bincode::{Decode, Encode};

use crate::utils::Snapshot;

#[derive(Encode, Decode)]
pub struct TxoutIndexToTxoutValue(BTreeMap<usize, f64>);

impl TxoutIndexToTxoutValue {
    pub fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl Snapshot<Self> for TxoutIndexToTxoutValue {
    fn name<'a>() -> &'a str {
        "height_to_aged__txout_index_to_txout_value"
    }
}

impl Deref for TxoutIndexToTxoutValue {
    type Target = BTreeMap<usize, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxoutIndexToTxoutValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
