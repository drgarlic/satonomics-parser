use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TxoutIndex {
    pub tx_index: u32,
    pub vout: u16,
}

impl TxoutIndex {
    pub fn new(tx_index: u32, vout: u16) -> Self {
        Self { tx_index, vout }
    }
}
