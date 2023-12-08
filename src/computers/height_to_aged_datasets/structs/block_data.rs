use std::sync::RwLock;

use bitcoin_explorer::FTransaction;
use nohash_hasher::IntMap;
use serde::{Deserialize, Serialize};

use crate::{structs::Outputs, utils::ftransaction_to_outputs};

pub struct BlockData {
    pub price: f32,
    pub amount: RwLock<f64>,
    pub txid_index_to_outputs: RwLock<IntMap<usize, RwLock<Outputs>>>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedBlockData(f32, IntMap<usize, Outputs>);

impl BlockData {
    pub fn new(price: f32) -> Self {
        Self {
            price,
            amount: RwLock::new(0.0),
            txid_index_to_outputs: RwLock::new(IntMap::default()),
        }
    }

    pub fn import(serialized: &SerializedBlockData) -> Self {
        let txid_index_to_outputs: IntMap<usize, RwLock<Outputs>> = serialized
            .1
            .iter()
            .map(|(txid_index, outputs)| (txid_index.to_owned(), RwLock::new(outputs.to_owned())))
            .collect();

        let amount = txid_index_to_outputs
            .values()
            .map(|outputs| outputs.read().unwrap().values().sum::<f64>())
            .sum::<f64>();

        Self {
            price: serialized.0,
            amount: RwLock::new(amount),
            txid_index_to_outputs: RwLock::new(txid_index_to_outputs),
        }
    }

    pub fn insert_outputs(&self, txid_index: usize, tx: &FTransaction) {
        self.txid_index_to_outputs
            .write()
            .unwrap()
            .insert(txid_index, {
                let outputs = ftransaction_to_outputs(tx);

                *self.amount.write().unwrap() += outputs.values().sum::<f64>();

                RwLock::new(outputs)
            });
    }

    pub fn serialize(&self) -> SerializedBlockData {
        let price = self.price;

        let outputs = self
            .txid_index_to_outputs
            .read()
            .unwrap()
            .iter()
            .map(|(index, outputs)| (index.to_owned(), outputs.read().unwrap().to_owned()))
            .collect();

        SerializedBlockData(price, outputs)
    }
}
