use std::{
    collections::BTreeMap,
    mem,
    ops::{Deref, DerefMut},
};

use rayon::prelude::*;

use crate::parse::{EmptyAddressData, SizedDatabase};

use super::{AnyDatabaseGroup, Metadata};

type Key = u32;
type Value = EmptyAddressData;
type Database = SizedDatabase<Key, Value>;

pub struct AddressIndexToEmptyAddressData {
    map: BTreeMap<usize, Database>,
    pub metadata: Metadata,
}

impl Deref for AddressIndexToEmptyAddressData {
    type Target = BTreeMap<usize, Database>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for AddressIndexToEmptyAddressData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

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

impl AnyDatabaseGroup for AddressIndexToEmptyAddressData {
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
        "address_index_to_empty_address_data"
    }
}
