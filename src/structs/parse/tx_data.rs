use bincode::{Decode, Encode};

use super::BlockPath;

#[derive(Debug, Encode, Decode)]
pub struct TxData {
    pub block_path: BlockPath,
    pub spendable_outputs: u16,
}

impl TxData {
    pub fn new(block_path: BlockPath, spendable_outputs: u16) -> Self {
        Self {
            block_path,
            spendable_outputs,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.spendable_outputs == 0
    }
}
