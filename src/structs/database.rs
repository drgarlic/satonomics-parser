use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs,
};

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};
use itertools::Itertools;

// https://docs.rs/sanakirja/latest/sanakirja/index.html
// https://pijul.org/posts/2021-02-06-rethinking-sanakirja/
// Seems indeed much faster than ReDB and LMDB (heed)
// But a lot has changed code wise between them so a retest wouldn't hurt
use sanakirja::{
    btree::{self, page, page_unsized, BTreeMutPage, Db_},
    direct_repr, Commit, Env, Error, MutTxn, RootDb, Storable, UnsizedStorable,
};

use crate::traits::SNAPSHOTS_FOLDER;

pub type SizedDatabase<Key, Value> = Database<Key, Key, Value, page::Page<Key, Value>>;
pub type UnsizedDatabase<KeyTree, KeyDB, Value> =
    Database<KeyTree, KeyDB, Value, page_unsized::Page<KeyDB, Value>>;

pub struct Database<KeyTree, KeyDB, Value, Page>
where
    KeyTree: Ord + Clone + Debug + Encode + Decode,
    KeyDB: Ord + ?Sized + Storable,
    Value: Copy + Storable + PartialEq,
    Page: BTreeMutPage<KeyDB, Value>,
{
    cached_gets: BTreeMap<KeyTree, Value>,
    cached_puts: BTreeMap<KeyTree, Value>,
    cached_dels: BTreeSet<KeyTree>,
    db: Db_<KeyDB, Value, Page>,
    txn: MutTxn<Env, ()>,
    key_tree_to_key_db: fn(&KeyTree) -> &KeyDB,
}

pub const SANAKIRJA_MAX_KEY_SIZE: usize = 510;
const ROOT_DB: usize = 0;
const PAGE_SIZE: u64 = 4096 * 256; // 1mo - Must be a multiplier of 4096

impl<KeyDB, KeyTree, Value, Page> Database<KeyTree, KeyDB, Value, Page>
where
    KeyTree: Ord + Clone + Debug + Encode + Decode,
    KeyDB: Ord + ?Sized + Storable,
    Value: Copy + Storable + PartialEq,
    Page: BTreeMutPage<KeyDB, Value>,
{
    pub fn open(
        folder: &str,
        file: &str,
        key_tree_to_key_db: fn(&KeyTree) -> &KeyDB,
    ) -> color_eyre::Result<Self> {
        let mut txn = Self::init_txn(folder, file)?;

        let db = txn
            .root_db(ROOT_DB)
            .unwrap_or_else(|| btree::create_db_(&mut txn).unwrap());

        Ok(Self {
            cached_gets: BTreeMap::default(),
            cached_puts: BTreeMap::default(),
            cached_dels: BTreeSet::default(),
            db,
            txn,
            key_tree_to_key_db,
        })
    }

    pub fn get(&mut self, key: &KeyTree) -> Option<&Value> {
        if let Some(cached_put) = self.cached_puts.get(key) {
            return Some(cached_put);
        }

        // Rust issue: &mut self borrow conflicting with itself
        // https://github.com/rust-lang/rust/issues/21906#issuecomment-73296543
        // Waiting for Polonius
        if self.cached_gets.contains_key(key) {
            return self.cached_gets.get(key);
        }

        let k = (self.key_tree_to_key_db)(key);

        let option = btree::get(&self.txn, &self.db, k, None).unwrap();

        if let Some((k_found, v)) = option {
            if k == k_found {
                self.cached_gets.insert(key.clone(), *v);
                return Some(v);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &KeyTree) {
        if self.cached_puts.remove(key).is_none() {
            self.cached_gets.remove(key);
            self.cached_dels.insert(key.clone());
        }
    }

    pub fn take(&mut self, key: &KeyTree) -> Option<Value> {
        if self.cached_dels.get(key).is_none() {
            self.cached_puts.remove(key).or({
                self.cached_dels.insert(key.clone());
                self.cached_gets.remove(key).or_else(|| {
                    // TODO: Quasi duplicate from `get`, fix it after polonius update
                    let k = (self.key_tree_to_key_db)(key);

                    let option = btree::get(&self.txn, &self.db, k, None).unwrap();

                    if let Some((k_found, v)) = option {
                        if k == k_found {
                            return Some(v.to_owned());
                        }
                    }

                    None
                })
            })
        } else {
            dbg!(key);
            panic!("Can't take twice");
        }
    }

    pub fn insert(&mut self, key: KeyTree, value: Value) -> Option<Value> {
        self.cached_dels.remove(&key);
        self.cached_puts.insert(key, value)
    }

    pub fn export(mut self) -> color_eyre::Result<(), Error> {
        self.cached_dels
            .into_iter()
            // Faster DB actions when keys are sorted
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
            // Faster DB actions when keys are sorted
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

        self.txn.set_root(ROOT_DB, self.db.db);

        self.txn.commit()
    }

    fn init_txn(folder: &str, file: &str) -> color_eyre::Result<MutTxn<Env, ()>> {
        let path = {
            if folder.is_empty() {
                SNAPSHOTS_FOLDER.to_owned()
            } else {
                format!("{SNAPSHOTS_FOLDER}/{folder}")
            }
        };

        fs::create_dir_all(&path)?;

        let env = unsafe { Env::new_nolock(format!("{path}/{file}"), PAGE_SIZE, 1).unwrap() };

        let txn = Env::mut_txn_begin(env)?;

        Ok(txn)
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, Default, Copy, Encode, Decode,
)]
pub struct U8x19([u8; 19]);
direct_repr!(U8x19);
impl From<&[u8]> for U8x19 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = Self::default();
        arr.copy_from_slice(slice);
        arr
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut, Default, Copy, Encode, Decode,
)]
pub struct U8x31([u8; 31]);
direct_repr!(U8x31);
impl From<&[u8]> for U8x31 {
    fn from(slice: &[u8]) -> Self {
        let mut arr = Self::default();
        arr.copy_from_slice(slice);
        arr
    }
}
