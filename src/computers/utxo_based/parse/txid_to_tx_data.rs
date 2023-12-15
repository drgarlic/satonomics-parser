use std::ops::{Deref, DerefMut};

use bincode::{Decode, Encode};

use crate::{
    structs::TxidMap,
    utils::{export_snapshot_bin, import_snapshot_map},
};

use super::TxData;

#[derive(Encode, Decode)]
pub struct TxidToTxData(TxidMap<TxData>);

const SNAPSHOT_NAME: &str = "height_to_aged__txid_to_tx_data";

impl TxidToTxData {
    pub fn new() -> Self {
        Self(TxidMap::new())
    }

    pub fn import() -> color_eyre::Result<Self> {
        import_snapshot_map::<Self>(SNAPSHOT_NAME)
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        export_snapshot_bin(SNAPSHOT_NAME, &self)
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
