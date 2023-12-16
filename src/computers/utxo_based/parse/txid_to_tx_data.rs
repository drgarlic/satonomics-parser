use std::ops::{Deref, DerefMut};

use bincode::{Decode, Encode};

use crate::{structs::TxidMap, utils::Snapshot};

use super::TxData;

#[derive(Encode, Decode)]
pub struct TxidToTxData(TxidMap<TxData>);

impl TxidToTxData {
    pub fn default() -> Self {
        Self(TxidMap::new())
    }
}

impl Snapshot<Self> for TxidToTxData {
    fn name<'a>() -> &'a str {
        "height_to_aged__txid_to_tx_data"
    }
}

impl Deref for TxidToTxData {
    type Target = TxidMap<TxData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxidToTxData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
