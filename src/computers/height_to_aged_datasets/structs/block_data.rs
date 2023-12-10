use serde::{Deserialize, Serialize};

pub struct BlockData {
    pub price: f32,
    pub amount: f64,
    pub outputs_len: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedBlockData(f32, f64, u32);

impl BlockData {
    pub fn new(price: f32) -> Self {
        Self {
            price,
            amount: 0.0,
            outputs_len: 0,
        }
    }

    pub fn import(serialized: &SerializedBlockData) -> Self {
        Self {
            price: serialized.0,
            amount: serialized.1,
            outputs_len: serialized.2,
        }
    }

    pub fn serialize(&self) -> SerializedBlockData {
        SerializedBlockData(self.price, self.amount, self.outputs_len)
    }
}
