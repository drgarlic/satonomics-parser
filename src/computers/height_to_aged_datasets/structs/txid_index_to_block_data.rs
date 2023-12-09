use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use itertools::Itertools;
use nohash_hasher::IntMap;
use rayon::prelude::*;

use crate::utils::{export_snapshot, import_snapshot_map};

use super::{BlockData, BlockDatasPerDay};

/// `txid_index` => `txout_index` at `vout` 0
pub struct TxidIndexToBlockData(IntMap<usize, Arc<BlockData>>);

impl Deref for TxidIndexToBlockData {
    type Target = IntMap<usize, Arc<BlockData>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxidIndexToBlockData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const SNAPSHOT_NAME: &str = "height_to_aged__txid_index_to_block_data";

impl TxidIndexToBlockData {
    pub fn import(block_datas_per_day: &BlockDatasPerDay) -> color_eyre::Result<Self> {
        let height_to_block_data = block_datas_per_day
            .par_iter()
            .flat_map(|date_data| {
                date_data
                    .blocks
                    .read()
                    .unwrap()
                    .iter()
                    .map(|block_data| (block_data.height, Arc::clone(block_data)))
                    .collect_vec()
            })
            .collect::<IntMap<_, _>>();

        let map = import_snapshot_map::<u32>(SNAPSHOT_NAME, true)?
            .into_iter()
            .map(|(txid_index, block_height)| {
                (
                    txid_index.parse::<usize>().unwrap(),
                    height_to_block_data
                        .get(&block_height)
                        .map(Arc::clone)
                        .unwrap(),
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
                .map(|(txid_index, block_data)| {
                    (txid_index.to_owned(), block_data.height.to_owned())
                })
                .collect::<IntMap<_, _>>(),
            false,
        )
    }
}
