use savefile_derive::Savefile;

use super::BlockPath;

#[derive(Debug, Savefile)]
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

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.spendable_outputs == 0
    }
}
