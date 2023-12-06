use std::cell::{Ref, RefCell, RefMut};

use bitcoin_explorer::{FTransaction, Txid};
use nohash_hasher::IntMap;

use crate::utils::ftransaction_to_outputs;

use super::{TxidHashMap, TxidMap};

pub type Outputs = IntMap<u32, f64>;

type V = RefCell<Outputs>;

pub struct TxidToOutputs(TxidMap<V>);

impl TxidToOutputs {
    pub fn new(max_size: Option<usize>) -> Self {
        Self(TxidMap::new(max_size))
    }

    pub fn insert(&self, tx: &FTransaction) -> Option<V> {
        self.0
            .insert(tx.txid, RefCell::new(ftransaction_to_outputs(tx)))
    }

    pub fn borrow_map(&self) -> Ref<'_, TxidHashMap<V>> {
        self.0.borrow_map()
    }

    pub fn borrow_mut_map(&self) -> RefMut<'_, TxidHashMap<V>> {
        self.0.borrow_mut_map()
    }

    pub fn take(&self, txid: &Txid, vout: &u32) -> Option<f64> {
        let mut txid_to_outputs = self.0.borrow_mut_map();

        if let Some(outputs) = txid_to_outputs.get(txid) {
            let mut value_opt = None;

            let remove = {
                let mut outputs = outputs.borrow_mut();

                outputs
                    .remove(vout)
                    .and_then(|value| value_opt.replace(value));

                outputs.is_empty()
            };

            if remove {
                txid_to_outputs.remove(txid);

                if self.0.is_finite() {
                    let mut ordered_txids = self.0.borrow_mut_ordered_txids();

                    let position = ordered_txids
                        .iter()
                        .position(|_txid| _txid == txid)
                        .expect("Txid to be in VecDeque since it was found in the map");

                    ordered_txids
                        .remove(position)
                        .expect("Being able to remove Txid in VecDeque");
                }
            }

            value_opt
        } else {
            None
        }
    }
}
