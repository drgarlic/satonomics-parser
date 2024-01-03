// Doesn't work WHYYYYY

use std::thread;

use bitcoin::Txid;
use rayon::prelude::*;
use sanakirja::{btree::page, Error};

use crate::{
    structs::{Database, U8_32},
    traits::Databases,
};

type Key = U8_32;
type Value = u32;
type Db = Database<Key, Key, Value, page::Page<Key, Value>>;

pub struct TxidToTxIndex {
    v1: Option<Db>,
    v2: Option<Db>,
    v3: Option<Db>,
}

impl TxidToTxIndex {
    pub fn insert(&mut self, txid: &Txid, tx_version: i32, tx_index: Value) -> Option<Value> {
        let version = tx_version as u8;

        let slice = Self::txid_to_slice(txid);

        self.get_db(version)
            .and_then(|db| db.insert(slice, tx_index))
    }

    pub fn get(&mut self, txid: &Txid) -> Option<Value> {
        self.find_db(txid).and_then(|(_, value)| value)
    }

    pub fn remove(&mut self, txid: &Txid) {
        if let Some((db, _)) = self.find_db(txid) {
            db.remove(&Self::txid_to_slice(txid));
        }
    }

    fn find_db(&mut self, txid: &Txid) -> Option<(&mut Db, Option<Value>)> {
        [(&mut self.v1, 1), (&mut self.v2, 2), (&mut self.v3, 3)]
            .into_par_iter()
            .map(|(db, version)| {
                let db = db.get_or_insert_with(|| Self::open_db(version));

                let value = db.get(&Self::txid_to_slice(txid)).cloned();

                (db, value)
            })
            .find_first(|(_, value)| value.is_some())
    }

    fn txid_to_slice(txid: &Txid) -> U8_32 {
        U8_32::from(&txid[..])
    }

    fn get_db(&mut self, version: u8) -> Option<&mut Db> {
        match version {
            1 => Some(self.v1.get_or_insert_with(|| Self::open_db(version))),
            2 => Some(self.v2.get_or_insert_with(|| Self::open_db(version))),
            3 => Some(self.v3.get_or_insert_with(|| Self::open_db(version))),
            _ => panic!("txid_to_index doesn't support transaction with a version higher than 3"),
        }
    }

    fn open_db(version: u8) -> Db {
        let path = format!("{}/v{}", Self::folder(), version);

        Database::open(&path, |key| key).unwrap()
    }
}

impl Databases for TxidToTxIndex {
    fn open(height: usize) -> color_eyre::Result<Self> {
        if height == 0 {
            let _ = Self::clear();
        }

        Ok(Self {
            v1: None,
            v2: None,
            v3: None,
        })
    }

    fn export(self) -> color_eyre::Result<(), Error> {
        thread::scope(|s| {
            s.spawn(|| self.v1.map(|db| db.export()));
            s.spawn(|| self.v2.map(|db| db.export()));
            s.spawn(|| self.v3.map(|db| db.export()));
        });

        Ok(())
    }

    fn folder<'a>() -> &'a str {
        "txid_to_tx_index"
    }
}
