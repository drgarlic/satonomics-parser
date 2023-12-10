use std::ops::{Deref, DerefMut};

use nohash_hasher::IntMap;

use crate::utils::{export_snapshot, import_snapshot_map};

use super::BlockDatasPerDay;

/// `txid_index` => `txout_index` at `vout` 0
pub struct TxidIndexToBlockData(IntMap<usize, u32>);

impl Deref for TxidIndexToBlockData {
    type Target = IntMap<usize, u32>;

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
const BLOCK_INDEX_SHIFT: u8 = 10;

impl TxidIndexToBlockData {
    pub fn import() -> color_eyre::Result<Self> {
        let map = import_snapshot_map::<u32>(SNAPSHOT_NAME, true)?
            .into_iter()
            .map(|(txid_index, block_index)| (txid_index.parse::<usize>().unwrap(), block_index))
            .collect();

        Ok(Self(map))
    }

    pub fn insert(&mut self, txid_index: usize, block_datas_per_day: &BlockDatasPerDay) {
        let day_index = block_datas_per_day.len() - 1;
        let block_index = block_datas_per_day.last().unwrap().blocks.len() - 1;

        let merged_index = (day_index << BLOCK_INDEX_SHIFT) + block_index;

        self.0.insert(txid_index, merged_index as u32);
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        export_snapshot(
            SNAPSHOT_NAME,
            &self
                .iter()
                .map(|(txid_index, block_index)| (txid_index.to_owned(), block_index.to_owned()))
                .collect::<IntMap<_, _>>(),
            false,
        )
    }

    pub fn separate_merged_indexes(index: &u32) -> (usize, usize) {
        let date_index = *index as usize >> BLOCK_INDEX_SHIFT;
        let block_index = *index as usize & (2_usize.pow(BLOCK_INDEX_SHIFT as u32) - 1);

        (date_index, block_index)
    }
}
