use std::ops::{Deref, DerefMut};

use bincode::{Decode, Encode};

use crate::{structs::TxidMap, traits::Snapshot};

#[derive(Encode, Decode, Default)]
pub struct TxidToTxIndex(TxidMap<u32>);

impl TxidToTxIndex {
    pub fn max_index(&self) -> u32 {
        self.values().max().map(|index| index + 1).unwrap_or(0)
    }
}

impl Snapshot for TxidToTxIndex {
    fn name<'a>() -> &'a str {
        "height_to_aged__txid_to_tx_index"
    }
}

impl Deref for TxidToTxIndex {
    type Target = TxidMap<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxidToTxIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
