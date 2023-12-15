use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bincode::{Decode, Encode};

use crate::utils::{export_snapshot_bin, import_snapshot_map};

use super::BlockPath;

#[derive(Encode, Decode)]
/// `txid_index` => `txout_index` at `vout` 0
pub struct TxidIndexToBlockPath(BTreeMap<usize, BlockPath>);

const SNAPSHOT_NAME: &str = "height_to_aged__txid_index_to_block_path";

impl TxidIndexToBlockPath {
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

impl Deref for TxidIndexToBlockPath {
    type Target = BTreeMap<usize, BlockPath>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxidIndexToBlockPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
