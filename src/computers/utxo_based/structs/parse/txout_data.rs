use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub struct TxoutData {
    pub value: f64,
    pub address_index: u32,
}

impl TxoutData {
    pub fn new(value: f64, address_index: u32) -> Self {
        Self {
            value,
            address_index,
        }
    }
}
