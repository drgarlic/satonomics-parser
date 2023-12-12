use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::utils::{export_snapshot, import_snapshot_map};

use super::BlockPath;

/// `txid_index` => `txout_index` at `vout` 0
pub struct TxidIndexToBlockPath(BTreeMap<usize, BlockPath>);

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

const SNAPSHOT_NAME: &str = "height_to_aged__txid_index_to_block_path";

impl TxidIndexToBlockPath {
    pub fn import() -> color_eyre::Result<Self> {
        let map = import_snapshot_map::<u32>(SNAPSHOT_NAME, true)?
            .into_iter()
            .map(|(txid_index, block_index)| {
                (
                    txid_index.parse::<usize>().unwrap(),
                    BlockPath::new(block_index),
                )
            })
            .collect();

        Ok(Self(map))
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        export_snapshot(
            SNAPSHOT_NAME,
            &self
                .iter()
                .map(|(txid_index, block_path)| (txid_index.to_owned(), *block_path.to_owned()))
                .collect::<BTreeMap<_, _>>(),
            false,
        )
    }
}
