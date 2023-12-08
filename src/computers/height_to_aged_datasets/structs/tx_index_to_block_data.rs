use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use itertools::Itertools;
use nohash_hasher::IntMap;

use super::{BlockData, BlockDatasPerDay};

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

impl TxidIndexToBlockData {
    pub fn from(block_datas_per_day: &BlockDatasPerDay) -> Self {
        Self(
            block_datas_per_day
                .iter()
                .flat_map(|date_data| {
                    date_data
                        .blocks
                        .read()
                        .unwrap()
                        .iter()
                        .flat_map(|block_data| {
                            block_data
                                .txid_index_to_outputs
                                .read()
                                .unwrap()
                                .iter()
                                .map(|(txid_index, _)| {
                                    (txid_index.to_owned(), Arc::clone(block_data))
                                })
                                .collect_vec()
                        })
                        .collect_vec()
                })
                .collect(),
        )
    }
}
