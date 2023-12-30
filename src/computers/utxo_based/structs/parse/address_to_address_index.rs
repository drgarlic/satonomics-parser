use std::collections::BTreeMap;

use itertools::Itertools;
use sanakirja::{
    btree::{self, UDb},
    Commit, Env, Error, MutTxn,
};

use super::EnvSanakirja;

pub struct AddressToAddressIndex {
    cache: BTreeMap<Vec<u8>, u32>,
    db: UDb<[u8], u32>,
    txn: MutTxn<Env, ()>,
}

impl AddressToAddressIndex {
    pub fn open(height: usize) -> color_eyre::Result<Self> {
        let env = {
            let name = "address_to_address_index";
            if height == 0 {
                EnvSanakirja::default(name)
            } else {
                EnvSanakirja::import(name)?
            }
        };

        let mut txn = Env::mut_txn_begin(env)?;

        let db = btree::create_db_(&mut txn)?;

        Ok(Self {
            cache: BTreeMap::default(),
            db,
            txn,
        })
    }

    pub fn get(&mut self, key: &[u8]) -> Option<u32> {
        self.cache
            .get(key)
            .cloned()
            .or(btree::get(&self.txn, &self.db, key, None)
                .unwrap()
                .map(|(_, v)| *v))
    }

    pub fn put(&mut self, key: &[u8], value: u32) {
        self.cache.insert(key.to_vec(), value);
    }

    pub fn export(mut self) -> Result<(), Error> {
        self.cache
            .into_iter()
            .sorted_unstable_by_key(|x| x.0.clone())
            .try_for_each(|(key, value)| -> Result<(), Error> {
                btree::put(&mut self.txn, &mut self.db, &key, &value)?;
                Ok(())
            })?;

        self.txn.commit()
    }
}
