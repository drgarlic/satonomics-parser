use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode)]
pub struct BlockData {
    pub height: u32,
    pub price: f32,
    pub amount: f64,
    pub outputs_len: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedBlockData(u32, f32, f64, u32);

impl BlockData {
    pub fn new(height: u32, price: f32) -> Self {
        Self {
            height,
            price,
            amount: 0.0,
            outputs_len: 0,
        }
    }

    pub fn import(serialized: &SerializedBlockData) -> Self {
        Self {
            height: serialized.0,
            price: serialized.1,
            amount: serialized.2,
            outputs_len: serialized.3,
        }
    }

    pub fn serialize(&self) -> SerializedBlockData {
        SerializedBlockData(self.height, self.price, self.amount, self.outputs_len)
    }
}
