use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use sanakirja::{
    btree::{self, Db},
    Commit, Env, Error, MutTxn,
};

use super::{EmptyAddressData, EnvSanakirja};

pub struct AddressIndexToEmptyAddressData {
    cache_put: BTreeMap<u32, EmptyAddressData>,
    cache_delete: BTreeSet<u32>,
    db: Db<u32, EmptyAddressData>,
    txn: MutTxn<Env, ()>,
}

impl AddressIndexToEmptyAddressData {
    pub fn open(height: usize) -> color_eyre::Result<Self> {
        let env = {
            let name = "address_index_to_empty_address_data";
            if height == 0 {
                EnvSanakirja::default(name)
            } else {
                EnvSanakirja::import(name)?
            }
        };

        let mut txn = Env::mut_txn_begin(env)?;

        let db = btree::create_db(&mut txn)?;

        Ok(Self {
            cache_put: BTreeMap::default(),
            cache_delete: BTreeSet::default(),
            db,
            txn,
        })
    }

    pub fn take(&mut self, key: &u32) -> Option<EmptyAddressData> {
        self.cache_put.remove(key).or({
            self.cache_delete.insert(*key);

            btree::get(&self.txn, &self.db, key, None)
                .unwrap()
                .map(|(_, v)| *v)
        })
    }

    pub fn put(&mut self, key: u32, value: EmptyAddressData) {
        self.cache_delete.remove(&key);
        self.cache_put.insert(key, value);
    }

    pub fn export(mut self) -> Result<(), Error> {
        self.cache_put
            .into_iter()
            .sorted_unstable_by_key(|x| x.0.clone())
            .for_each(|(key, value)| {
                btree::put(&mut self.txn, &mut self.db, &key, &value).unwrap();
            });

        self.cache_delete
            .into_iter()
            .sorted_unstable()
            .for_each(|key| {
                btree::del(&mut self.txn, &mut self.db, &key, None).unwrap();
            });

        self.txn.commit()
    }
}
