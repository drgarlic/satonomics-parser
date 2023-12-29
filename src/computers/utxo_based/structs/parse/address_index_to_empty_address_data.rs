use std::collections::{BTreeMap, BTreeSet};

use heed::{
    byteorder::NativeEndian,
    types::{SerdeBincode, U32},
    Database, Result, RoTxn, RwTxn,
};
use itertools::Itertools;

use super::{EmptyAddressData, HeedEnv};

type Key = U32<NativeEndian>;
type Value = SerdeBincode<EmptyAddressData>;
type DB = Database<Key, Value>;

pub struct AddressIndexToEmptyAddressData {
    cache_put: BTreeMap<u32, EmptyAddressData>,
    cache_delete: BTreeSet<u32>,
    db: DB,
}

impl AddressIndexToEmptyAddressData {
    pub fn open(env: &HeedEnv, writer: &mut RwTxn) -> Result<Self> {
        let db = env
            .create_database(writer, Some("address_index_to_empty_address_data"))
            .unwrap();

        Ok(Self {
            cache_put: BTreeMap::default(),
            cache_delete: BTreeSet::default(),
            db,
        })
    }

    pub fn take(&mut self, reader: &RoTxn, key: &u32) -> Option<EmptyAddressData> {
        self.cache_put.remove(key).or({
            self.cache_delete.insert(*key);
            self.db.get(reader, key).unwrap()
        })
    }

    pub fn put(&mut self, key: u32, value: EmptyAddressData) {
        self.cache_delete.remove(&key);
        self.cache_put.insert(key, value);
    }

    pub fn commit(&mut self, writer: &mut RwTxn) {
        self.cache_put
            .iter()
            .sorted_unstable_by_key(|x| x.0)
            .for_each(|(key, data)| self.db.put(writer, key, data).unwrap());

        self.cache_put.clear();

        self.cache_delete.iter().sorted_unstable().for_each(|key| {
            self.db.delete(writer, key).unwrap();
        });

        self.cache_delete.clear();
    }
}
