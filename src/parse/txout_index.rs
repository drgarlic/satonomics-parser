use savefile_derive::Savefile;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Savefile)]
pub struct TxoutIndex {
    pub tx_index: u32,
    pub vout: u16,
}

impl TxoutIndex {
    pub fn new(tx_index: u32, vout: u16) -> Self {
        Self { tx_index, vout }
    }
}
