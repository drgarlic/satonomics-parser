use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TxData {
    pub txid_index: usize,
    pub outputs_len: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedTxData(usize, u32);

impl TxData {
    pub fn new(txid_index: usize, outputs_len: u32) -> Self {
        Self {
            txid_index,
            outputs_len,
        }
    }

    pub fn deserialize(serialized: &SerializedTxData) -> Self {
        Self {
            txid_index: serialized.0,
            outputs_len: serialized.1,
        }
    }

    pub fn serialize(&self) -> SerializedTxData {
        SerializedTxData(self.txid_index, self.outputs_len)
    }
}
