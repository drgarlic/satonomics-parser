use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bincode::{Decode, Encode};

use crate::traits::Snapshot;

use super::{TxoutData, TxoutIndex};

#[derive(Encode, Decode, Default)]
pub struct TxoutIndexToTxoutData(BTreeMap<TxoutIndex, TxoutData>);

impl Snapshot for TxoutIndexToTxoutData {
    fn name<'a>() -> &'a str {
        "height_to_aged__txout_index_to_txout_data"
    }
}

impl Deref for TxoutIndexToTxoutData {
    type Target = BTreeMap<TxoutIndex, TxoutData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxoutIndexToTxoutData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
