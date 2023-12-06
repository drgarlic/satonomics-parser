use std::cell::RefCell;

use nohash_hasher::IntMap;
use serde::{Deserialize, Serialize};

use crate::structs::Outputs;

pub struct BlockData {
    pub price: f32,
    pub txid_index_to_outputs: RefCell<IntMap<usize, RefCell<Outputs>>>,
}

pub struct SquashedBlockData {
    pub price: f32,
    pub amount: f64,
    pub utxo_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedBlockData(f32, IntMap<usize, Outputs>);

impl BlockData {
    pub fn new(price: f32) -> Self {
        Self {
            price,
            txid_index_to_outputs: RefCell::new(IntMap::default()),
        }
    }

    pub fn import(serialized: SerializedBlockData) -> Self {
        Self {
            price: serialized.0,
            txid_index_to_outputs: RefCell::new(
                serialized
                    .1
                    .iter()
                    .map(|(txid_index, outputs)| {
                        (txid_index.to_owned(), RefCell::new(outputs.to_owned()))
                    })
                    .collect(),
            ),
        }
    }

    pub fn squash(&self) -> SquashedBlockData {
        let amount = self
            .txid_index_to_outputs
            .borrow()
            .values()
            .map(|map| map.borrow().values().sum::<f64>())
            .sum();

        let utxo_count = self.txid_index_to_outputs.borrow().len();

        SquashedBlockData {
            amount,
            price: self.price,
            utxo_count,
        }
    }
}
