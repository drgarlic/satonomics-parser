use std::ops::{Deref, DerefMut};

use bincode::{Decode, Encode};

const BLOCK_INDEX_SHIFT: u8 = 10;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, Encode, Decode)]
pub struct BlockPath(u32);

pub struct SplitBlockPath {
    pub date_index: usize,
    pub block_index: usize,
}

impl BlockPath {
    pub fn new(block_path: u32) -> Self {
        Self(block_path)
    }

    pub fn build(date_index: usize, block_index: usize) -> Self {
        Self(Self::merge(date_index, block_index))
    }

    pub fn merge(date_index: usize, block_index: usize) -> u32 {
        ((date_index << BLOCK_INDEX_SHIFT) + block_index) as u32
    }

    pub fn split(&self) -> SplitBlockPath {
        let merged = self.0 as usize;

        let date_index = merged >> BLOCK_INDEX_SHIFT;

        let block_index = merged & (2_usize.pow(BLOCK_INDEX_SHIFT as u32) - 1);

        SplitBlockPath {
            date_index,
            block_index,
        }
    }
}

impl Deref for BlockPath {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BlockPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
