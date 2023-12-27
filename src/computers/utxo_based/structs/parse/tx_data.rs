use bincode::{Decode, Encode};

use super::BlockPath;

#[derive(Debug, Encode, Decode)]
pub struct TxData {
    pub block_path: BlockPath,
    pub outputs_len: u16,
}

impl TxData {
    pub fn new(block_path: BlockPath, outputs_len: u16) -> Self {
        Self {
            block_path,
            outputs_len,
        }
    }
}
