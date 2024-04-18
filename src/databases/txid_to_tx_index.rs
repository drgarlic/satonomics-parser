use std::{
    collections::BTreeMap,
    mem,
    ops::{Deref, DerefMut},
};

use bitcoin::Txid;
use rayon::prelude::*;

use crate::parse::{SizedDatabase, U8x31};

use super::{AnyDatabaseGroup, Metadata};

type Key = U8x31;
type Value = u32;
type Database = SizedDatabase<Key, Value>;

pub struct TxidToTxIndex {
    map: BTreeMap<u8, Database>,
    pub metadata: Metadata,
}

impl Deref for TxidToTxIndex {
    type Target = BTreeMap<u8, Database>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TxidToTxIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl TxidToTxIndex {
    pub fn insert(&mut self, txid: &Txid, tx_index: Value) -> Option<Value> {
        let txid_key = Self::txid_to_key(txid);
        self.open_db(txid).insert(txid_key, tx_index)
    }

    // pub fn safe_get(&mut self, txid: &Txid) -> Option<&Value> {
    //     let txid_key = Self::txid_to_key(txid);
    //     self.open_db(txid).get(&txid_key)
    // }

    /// Doesn't check if the database is open contrary to `safe_get` which does and opens if needed.
    /// Though it makes it easy to use with rayon
    pub fn unsafe_get(&self, txid: &Txid) -> Option<&Value> {
        let txid_key = Self::txid_to_key(txid);
        let db_index = Self::db_index(txid);
        self.get(&db_index).unwrap().get(&txid_key)
    }

    pub fn unsafe_get_from_puts(&self, txid: &Txid) -> Option<&Value> {
        let txid_key = Self::txid_to_key(txid);
        let db_index = Self::db_index(txid);
        self.get(&db_index).unwrap().get_from_puts(&txid_key)
    }

    pub fn remove(&mut self, txid: &Txid) {
        let txid_key = Self::txid_to_key(txid);
        self.open_db(txid).remove(&txid_key);
    }

    pub fn open_db(&mut self, txid: &Txid) -> &mut Database {
        let db_index = Self::db_index(txid);

        self.entry(db_index).or_insert_with(|| {
            SizedDatabase::open(Self::folder(), &db_index.to_string(), |key| key).unwrap()
        })
    }

    fn txid_to_key(txid: &Txid) -> U8x31 {
        U8x31::from(&txid[1..])
    }

    fn db_index(txid: &Txid) -> u8 {
        txid[0]
    }
}

impl AnyDatabaseGroup for TxidToTxIndex {
    fn import() -> Self {
        Self {
            map: BTreeMap::default(),
            metadata: Metadata::import(&Self::full_path()),
        }
    }

    fn export(&mut self) -> color_eyre::Result<()> {
        mem::take(&mut self.map)
            .into_par_iter()
            .try_for_each(|(_, db)| db.export())?;

        self.metadata.export()?;

        Ok(())
    }

    fn reset_metadata(&mut self) {
        self.metadata.reset();
    }

    fn folder<'a>() -> &'a str {
        "txid_to_tx_index"
    }
}
