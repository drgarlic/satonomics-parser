use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct TxData {
    pub txid_index: usize,
    pub outputs_len: u32,
}

impl TxData {
    pub fn new(txid_index: usize, outputs_len: u32) -> Self {
        Self {
            txid_index,
            outputs_len,
        }
    }
}
