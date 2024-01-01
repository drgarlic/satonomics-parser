use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs,
};

use derive_deref::{Deref, DerefMut};
use itertools::Itertools;
use sanakirja::{
    btree::{self, page, page_unsized, BTreeMutPage, Db_},
    direct_repr, Commit, Env, Error, MutTxn, Storable, UnsizedStorable,
};

use crate::traits::SNAPSHOTS_FOLDER;

// const ROOT_DB: usize = 0;

pub type SizedDatabase<Key, Value> = Database<Key, Key, Value, page::Page<Key, Value>>;
pub type UnsizedDatabase<KeyTree, KeyDB, Value> =
    Database<KeyTree, KeyDB, Value, page_unsized::Page<KeyDB, Value>>;

pub struct Database<KeyTree, KeyDB, Value, Page>
where
    KeyTree: Ord + Clone,
    KeyDB: Ord + ?Sized + Storable,
    Value: Copy + Storable + PartialEq,
    Page: BTreeMutPage<KeyDB, Value>,
{
    cached_puts: BTreeMap<KeyTree, Value>,
    cached_dels: BTreeSet<KeyTree>,
    db: Db_<KeyDB, Value, Page>,
    txn: MutTxn<Env, ()>,
    key_tree_to_key_db: fn(&KeyTree) -> &KeyDB,
}

impl<KeyDB, KeyTree, Value, Page> Database<KeyTree, KeyDB, Value, Page>
where
    KeyTree: Ord + Clone + Debug,
    KeyDB: Ord + ?Sized + Storable,
    Value: Copy + Storable + PartialEq,
    Page: BTreeMutPage<KeyDB, Value>,
{
    pub fn open(
        name: &str,
        key_tree_to_key_db: fn(&KeyTree) -> &KeyDB,
    ) -> color_eyre::Result<Self> {
        let mut txn = Self::init_txn(name)?;

        let db: Db_<KeyDB, Value, Page> = {
            // if height < 2544 {
            btree::create_db_(&mut txn).unwrap()
            // } else {
            //     txn.root_db(ROOT_DB).unwrap()
            // }
        };

        Ok(Self {
            cached_puts: BTreeMap::default(),
            cached_dels: BTreeSet::default(),
            db,
            txn,
            key_tree_to_key_db,
        })
    }

    pub fn get(&self, key: &KeyTree) -> Option<&Value> {
        self.cached_puts.get(key).or({
            btree::get(&self.txn, &self.db, (self.key_tree_to_key_db)(key), None)
        }
        .unwrap()
        .map(|(_, v)| v))
    }

    // pub fn remove(&mut self, key: &KeyTree) {
    //     if self.cached_puts.remove(key).is_none() {
    //         self.cached_dels.insert(key.clone());
    //     }
    // }

    pub fn take(&mut self, key: &KeyTree) -> Option<Value> {
        self.cached_puts.remove(key).or({
            self.cached_dels.insert(key.clone());

            btree::get(&self.txn, &self.db, (self.key_tree_to_key_db)(key), None)
                .unwrap()
                .map(|(_, v)| *v)
        })
    }

    pub fn insert(&mut self, key: KeyTree, value: Value) -> Option<Value> {
        self.cached_puts.insert(key, value)
    }

    pub fn export(mut self) -> color_eyre::Result<(), Error> {
        self.cached_dels
            .into_iter()
            .sorted_unstable()
            .try_for_each(|key| -> Result<(), Error> {
                btree::del(
                    &mut self.txn,
                    &mut self.db,
                    (self.key_tree_to_key_db)(&key),
                    None,
                )?;

                Ok(())
            })?;

        self.cached_puts
            .into_iter()
            .sorted_unstable_by_key(|x| x.0.clone())
            .try_for_each(|(key, value)| -> Result<(), Error> {
                btree::put(
                    &mut self.txn,
                    &mut self.db,
                    (self.key_tree_to_key_db)(&key),
                    &value,
                )?;

                Ok(())
            })?;

        // self.txn.set_root(ROOT_DB, self.db.db);

        self.txn.commit()
    }

    fn init_txn(name: &str) -> color_eyre::Result<MutTxn<Env, ()>> {
        let env = Self::import_env(name)?;

        let txn = Env::mut_txn_begin(env)?;

        Ok(txn)
    }

    // fn default_env(name: &str) -> Env {
    //     let _ = fs::remove_dir_all(Self::path_str(name));

    //     Self::import_env(name).unwrap()
    // }

    fn import_env(name: &str) -> color_eyre::Result<Env> {
        let path = Self::path_str(name);

        fs::create_dir_all(&path)?;

        let env = unsafe { Env::new_nolock(format!("{path}/db"), 4096 * 256, 1).unwrap() };

        Ok(env)
    }

    fn path_str(name: &str) -> String {
        format!("{SNAPSHOTS_FOLDER}/{name}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, Default)]
pub struct U8_20([u8; 20]);
direct_repr!(U8_20);
impl U8_20 {
    pub fn from(slice: &[u8]) -> Self {
        let mut arr = Self::default();
        arr.copy_from_slice(slice);
        arr
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, Default)]
pub struct U8_32([u8; 32]);
direct_repr!(U8_32);
impl U8_32 {
    pub fn from(slice: &[u8]) -> Self {
        let mut arr = Self::default();
        arr.copy_from_slice(slice);
        arr
    }
}
