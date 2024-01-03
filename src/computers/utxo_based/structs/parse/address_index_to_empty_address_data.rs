use derive_deref::{Deref, DerefMut};
use nohash_hasher::IntMap;
use rayon::prelude::*;
use sanakirja::Error;

use crate::{
    structs::{Database, SizedDatabase},
    traits::Databases,
};

use super::EmptyAddressData;

type Key = u32;
type Value = EmptyAddressData;
type Db = SizedDatabase<Key, Value>;

#[derive(Deref, DerefMut, Default)]
pub struct AddressIndexToEmptyAddressData(IntMap<usize, Db>);

const DB_MAX_SIZE: usize = 100_000_000;

impl AddressIndexToEmptyAddressData {
    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.open_db(&key).insert(key, value)
    }

    pub fn take(&mut self, key: &Key) -> Option<Value> {
        self.open_db(key).take(key)
    }

    fn open_db(&mut self, key: &Key) -> &mut Db {
        let db_index = Self::db_index(key);

        self.entry(db_index).or_insert_with(|| {
            let db_name = format!(
                "{}..{}",
                db_index * DB_MAX_SIZE,
                (db_index + 1) * DB_MAX_SIZE
            );

            Database::open(Self::folder(), &db_name, |key| key).unwrap()
        })
    }

    fn db_index(key: &Key) -> usize {
        *key as usize / DB_MAX_SIZE
    }
}

impl Databases for AddressIndexToEmptyAddressData {
    fn open(height: usize) -> color_eyre::Result<Self> {
        if height == 0 {
            let _ = Self::clear();
        }

        Ok(Self::default())
    }

    fn export(mut self) -> color_eyre::Result<(), Error> {
        self.par_drain().try_for_each(|(_, db)| db.export())
    }

    fn folder<'a>() -> &'a str {
        "address_index_to_empty_address_data"
    }
}
