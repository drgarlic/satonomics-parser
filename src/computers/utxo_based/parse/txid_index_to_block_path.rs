use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bincode::{Decode, Encode};

use crate::utils::Snapshot;

use super::BlockPath;

#[derive(Encode, Decode)]
/// `txid_index` => `txout_index` at `vout` 0
pub struct TxidIndexToBlockPath(BTreeMap<usize, BlockPath>);

impl TxidIndexToBlockPath {
    pub fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl Snapshot<Self> for TxidIndexToBlockPath {
    fn name<'a>() -> &'a str {
        "height_to_aged__txid_index_to_block_path"
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
