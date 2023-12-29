use std::collections::BTreeMap;

use heed::{
    byteorder::NativeEndian,
    types::{Bytes, U32},
    Database, Result, RoTxn, RwTxn,
};
use itertools::Itertools;

use super::HeedEnv;

/// Address is the string version of either:
/// - mono
/// - multi (sorted and joined)
// type Key = &'static [u8];
type Key = Bytes;
type Value = U32<NativeEndian>;
type DB = Database<Key, Value>;

pub struct AddressToAddressIndex {
    cache: BTreeMap<Vec<u8>, u32>,
    db: DB,
}

impl AddressToAddressIndex {
    pub fn open(env: &HeedEnv, writer: &mut RwTxn) -> Result<Self> {
        let db = env
            .create_database(writer, Some("address_index_to_address"))
            .unwrap();

        Ok(Self {
            cache: BTreeMap::default(),
            db,
        })
    }

    pub fn get(&mut self, reader: &RoTxn, key: &[u8]) -> Option<u32> {
        self.cache
            .get(key)
            .cloned()
            .or(self.db.get(reader, key).unwrap())
    }

    pub fn put(&mut self, key: &[u8], value: u32) {
        self.cache.insert(key.to_vec(), value);
    }

    pub fn len(&self, reader: &RoTxn) -> Result<u64> {
        self.db.len(reader)
    }

    pub fn commit(&mut self, writer: &mut RwTxn) {
        self.cache
            .iter()
            .sorted_unstable_by_key(|x| x.0)
            .for_each(|(key, data)| self.db.put(writer, key, data).unwrap());

        self.cache.clear();
    }
}
