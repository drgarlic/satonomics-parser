use bitcoin::Txid;
use sanakirja::{btree::page, Error};

use crate::{
    structs::{Database, U8_32},
    traits::Databases,
};

type Key = U8_32;
type Value = u32;
type Db = Database<Key, Key, Value, page::Page<Key, Value>>;

pub struct TxidToTxIndex(Db);

impl TxidToTxIndex {
    pub fn insert(&mut self, txid: &Txid, tx_index: Value) -> Option<Value> {
        self.0.insert(Self::txid_to_slice(txid), tx_index)
    }

    pub fn get(&mut self, txid: &Txid) -> Option<&Value> {
        self.0.get(&Self::txid_to_slice(txid))
    }

    pub fn remove(&mut self, txid: &Txid) {
        self.0.remove(&Self::txid_to_slice(txid));
    }

    fn txid_to_slice(txid: &Txid) -> U8_32 {
        U8_32::from(&txid[..])
    }
}

impl Databases for TxidToTxIndex {
    fn open(height: usize) -> color_eyre::Result<Self> {
        if height == 0 {
            let _ = Self::clear();
        }

        Ok(Self(Database::open(Self::folder(), |key| key).unwrap()))
    }

    fn export(self) -> color_eyre::Result<(), Error> {
        self.0.export()
    }

    fn folder<'a>() -> &'a str {
        "txid_to_tx_index"
    }
}
