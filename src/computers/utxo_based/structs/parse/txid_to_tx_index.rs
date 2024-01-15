use bitcoin::Txid;
use derive_deref::{Deref, DerefMut};
use nohash_hasher::IntMap;
use rayon::prelude::*;

use crate::{
    structs::{SizedDatabase, U8x31},
    traits::Databases,
};

type Key = U8x31;
type Value = u32;
type Db = SizedDatabase<Key, Value>;

#[derive(Deref, DerefMut, Default)]
pub struct TxidToTxIndex(IntMap<u8, Db>);

impl TxidToTxIndex {
    pub fn insert(&mut self, txid: &Txid, tx_index: Value) -> Option<Value> {
        let (txid_prefix, txid_rest) = Self::split_txid(txid);
        self.open_db(txid_prefix).insert(txid_rest, tx_index)
    }

    pub fn get(&mut self, txid: &Txid) -> Option<Value> {
        let (txid_prefix, txid_rest) = Self::split_txid(txid);
        self.open_db(txid_prefix).get(&txid_rest).cloned()
    }

    pub fn remove(&mut self, txid: &Txid) {
        let (txid_prefix, txid_rest) = Self::split_txid(txid);
        self.open_db(txid_prefix).remove(&txid_rest);
    }

    fn open_db(&mut self, txid_prefix: u8) -> &mut Db {
        self.entry(txid_prefix).or_insert_with(|| {
            SizedDatabase::open(Self::folder(), &txid_prefix.to_string(), |key| key).unwrap()
        })
    }

    fn split_txid(txid: &Txid) -> (u8, U8x31) {
        (txid[0], U8x31::from(&txid[1..]))
    }
}

impl Databases for TxidToTxIndex {
    fn open(height: usize) -> color_eyre::Result<Self> {
        if height == 0 {
            let _ = Self::clear();
        }

        Ok(Self::default())
    }

    fn export(mut self) -> color_eyre::Result<()> {
        self.par_drain().try_for_each(|(_, db)| db.export())?;

        Ok(())
    }

    fn folder<'a>() -> &'a str {
        "txid_to_tx_index"
    }
}
