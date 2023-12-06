use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use itertools::Itertools;
use nohash_hasher::IntMap;

use super::{BlockData, BlockDatasPerDay};

pub struct TxidIndexToBlockData(IntMap<usize, Rc<BlockData>>);

impl Deref for TxidIndexToBlockData {
    type Target = IntMap<usize, Rc<BlockData>>;

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
                        .borrow()
                        .iter()
                        .flat_map(|block_data| {
                            block_data
                                .txid_index_to_outputs
                                .borrow()
                                .iter()
                                .map(|(txid_index, _)| {
                                    (txid_index.to_owned(), Rc::clone(block_data))
                                })
                                .collect_vec()
                        })
                        .collect_vec()
                })
                .collect(),
        )
    }
}
