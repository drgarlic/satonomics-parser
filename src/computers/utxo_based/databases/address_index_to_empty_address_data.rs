use derive_deref::{Deref, DerefMut};
use nohash_hasher::IntMap;
use rayon::prelude::*;

use crate::{computers::utxo_based::EmptyAddressData, structs::SizedDatabase};

use super::DatabaseGroup;

type Key = u32;
type Value = EmptyAddressData;
type Database = SizedDatabase<Key, Value>;

#[derive(Deref, DerefMut, Default)]
pub struct AddressIndexToEmptyAddressData(IntMap<usize, Database>);

const DB_MAX_SIZE: usize = 1_000_000;

impl AddressIndexToEmptyAddressData {
    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.open_db(&key).insert(key, value)
    }

    pub fn remove_from_puts(&mut self, key: &Key) -> Option<Value> {
        self.open_db(key).remove_from_puts(key)
    }

    pub fn remove(&mut self, key: &Key) {
        self.open_db(key).remove(key)
    }

    /// Doesn't check if the database is open contrary to `safe_get` which does and opens if needed
    /// Though it makes it easy to use with rayon.
    pub fn unsafe_get(&self, key: &Key) -> Option<&Value> {
        let db_index = Self::db_index(key);

        self.get(&db_index).unwrap().get(key)
    }

    pub fn open_db(&mut self, key: &Key) -> &mut Database {
        let db_index = Self::db_index(key);

        self.entry(db_index).or_insert_with(|| {
            let db_name = format!(
                "{}..{}",
                db_index * DB_MAX_SIZE,
                (db_index + 1) * DB_MAX_SIZE
            );

            SizedDatabase::open(Self::folder(), &db_name, |key| key).unwrap()
        })
    }

    fn db_index(key: &Key) -> usize {
        *key as usize / DB_MAX_SIZE
    }
}

impl DatabaseGroup for AddressIndexToEmptyAddressData {
    fn drain_export(&mut self) -> color_eyre::Result<()> {
        self.par_drain().try_for_each(|(_, db)| db.export())?;

        Ok(())
    }

    fn folder<'a>() -> &'a str {
        "address_index_to_empty_address_data"
    }
}
