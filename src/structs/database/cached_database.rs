use std::fmt::Debug;

use bincode::{Decode, Encode};
use sanakirja::{
    btree::{page, page_unsized, BTreeMutPage},
    Storable,
};

use super::{cache::Cache, Database};

pub type SizedCachedDatabase<Key, Value> = CachedDatabase<Key, Key, Value, page::Page<Key, Value>>;
pub type UnsizedCachedDatabase<KeyTree, KeyDB, Value> =
    CachedDatabase<KeyTree, KeyDB, Value, page_unsized::Page<KeyDB, Value>>;

pub struct CachedDatabase<KeyTree, KeyDB, Value, Page>
where
    KeyTree: Ord + Clone + Debug + Encode + Decode,
    KeyDB: Ord + ?Sized + Storable,
    Value: Copy + Storable + PartialEq + Encode + Decode,
    Page: BTreeMutPage<KeyDB, Value>,
{
    cache: Cache<KeyTree, Value>,
    database: Database<KeyTree, KeyDB, Value, Page>,
}

impl<KeyDB, KeyTree, Value, Page> CachedDatabase<KeyTree, KeyDB, Value, Page>
where
    KeyTree: Ord + Clone + Debug + Encode + Decode,
    KeyDB: Ord + ?Sized + Storable,
    Value: Copy + Storable + PartialEq + Encode + Decode,
    Page: BTreeMutPage<KeyDB, Value>,
{
    pub fn open(
        folder: &str,
        file: &str,
        key_tree_to_key_db: fn(&KeyTree) -> &KeyDB,
    ) -> color_eyre::Result<Self> {
        // Database first to create path if needed
        let database = Database::open(folder, file, key_tree_to_key_db)?;

        let cache = Cache::import(folder, file)?;

        Ok(Self { cache, database })
    }

    pub fn get(&mut self, key: &KeyTree) -> Option<&Value> {
        self.cache.get(key).or_else(|| self.database.get(key))
    }

    pub fn remove(&mut self, key: &KeyTree) {
        if self.cache.remove(key).is_none() {
            self.database.remove(key);
        }
    }

    pub fn set_date_index(&mut self, date_index: i32) {
        self.cache.set_date_index(date_index);
    }

    pub fn take(&mut self, key: &KeyTree) -> Option<Value> {
        self.cache.remove(key).or_else(|| self.database.take(key))
    }

    pub fn insert(&mut self, key: KeyTree, value: Value) -> Option<Value> {
        self.cache.insert(key, value)
    }

    pub fn export(mut self) -> color_eyre::Result<()> {
        self.cache.export()?.into_iter().for_each(|(key, value)| {
            self.database.insert(key, value);
            // self.database.unsafe_insert(key, value);
        });

        self.database.export()?;

        Ok(())
    }
}
