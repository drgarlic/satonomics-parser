use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bincode::{Decode, Encode};

use crate::utils::{export_snapshot_bin, import_snapshot_map};

#[derive(Encode, Decode)]
pub struct TxoutIndexToTxoutValue(BTreeMap<usize, f64>);

const SNAPSHOT_NAME: &str = "height_to_aged__txout_index_to_txout_value";

impl TxoutIndexToTxoutValue {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn import() -> color_eyre::Result<Self> {
        import_snapshot_map::<Self>(SNAPSHOT_NAME)
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        export_snapshot_bin(SNAPSHOT_NAME, &self)
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
