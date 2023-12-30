use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use itertools::Itertools;
use sanakirja::{
    btree::{self, Db},
    Commit, Env, Error, MutTxn, Storable,
};

use super::SNAPSHOT_FOLDER;

pub trait DatabaseTrait<Key, Value>
where
    Key: Clone + Storable + Ord,
    Value: Copy + Storable + PartialEq,
{
    fn init_txn(name: &str, height: usize) -> color_eyre::Result<MutTxn<Env, ()>> {
        let env = {
            if height == 0 {
                default_env(name)
            } else {
                import_env(name)?
            }
        };

        let txn = Env::mut_txn_begin(env)?;

        Ok(txn)
    }

    fn txn(&self) -> &MutTxn<Env, ()>;

    fn db(&self) -> &Db<Key, Value>;

    fn cache_put(&mut self) -> &mut BTreeMap<Key, Value>;

    fn cache_delete(&mut self) -> &mut BTreeSet<Key>;

    #[allow(clippy::type_complexity)]
    fn consume(
        self,
    ) -> (
        MutTxn<Env, ()>,
        Db<Key, Value>,
        BTreeMap<Key, Value>,
        BTreeSet<Key>,
    );

    fn get(&mut self, key: &Key) -> Option<Value> {
        self.cache_put()
            .get(key)
            .cloned()
            .or(btree::get(self.txn(), self.db(), key, None)
                .unwrap()
                .map(|(_, v)| *v))
    }

    fn take(&mut self, key: &Key) -> Option<Value> {
        self.cache_put().remove(key).or({
            self.cache_delete().insert(key.clone());

            btree::get(self.txn(), self.db(), key, None)
                .unwrap()
                .map(|(_, v)| *v)
        })
    }

    fn put(&mut self, key: Key, value: Value) {
        self.cache_delete().remove(&key);
        self.cache_put().insert(key, value);
    }

    fn export(self) -> Result<(), Error>
    where
        Self: std::marker::Sized,
    {
        let (mut txn, mut db, cache_put, cache_delete) = self.consume();

        cache_put
            .into_iter()
            .sorted_unstable_by_key(|x| x.0.clone())
            .try_for_each(|(key, value)| -> Result<(), Error> {
                btree::put(&mut txn, &mut db, &key, &value)?;
                Ok(())
            })?;

        cache_delete.into_iter().sorted_unstable().for_each(|key| {
            btree::del(&mut txn, &mut db, &key, None).unwrap();
        });

        txn.commit()
    }
}

fn import_env(name: &str) -> color_eyre::Result<Env> {
    let str = format!("{SNAPSHOT_FOLDER}/{name}");

    let path = Path::new(&str);

    fs::create_dir_all(path)?;

    let env = Env::new(path.join(Path::new("db")), 4096 * 1000, 1).unwrap();

    Ok(env)
}

fn default_env(name: &str) -> Env {
    let _ = fs::remove_dir_all(format!("{SNAPSHOT_FOLDER}/{name}"));

    import_env(name).unwrap()
}
