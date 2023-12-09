use std::sync::RwLock;

use serde::{Deserialize, Serialize};

pub struct BlockData {
    pub height: u32,
    pub price: f32,
    pub amount: RwLock<f64>,
    pub outputs_len: RwLock<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedBlockData(u32, f32, f64, u32);

impl BlockData {
    pub fn new(height: usize, price: f32) -> Self {
        Self {
            height: height as u32,
            price,
            amount: RwLock::new(0.0),
            outputs_len: RwLock::new(0),
        }
    }

    pub fn import(serialized: &SerializedBlockData) -> Self {
        Self {
            height: serialized.0,
            price: serialized.1,
            amount: RwLock::new(serialized.2),
            outputs_len: RwLock::new(serialized.3),
        }
    }

    pub fn serialize(&self) -> SerializedBlockData {
        SerializedBlockData(
            self.height,
            self.price,
            self.amount.read().unwrap().to_owned(),
            self.outputs_len.read().unwrap().to_owned(),
        )
    }
}
