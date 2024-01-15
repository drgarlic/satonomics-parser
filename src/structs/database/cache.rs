use std::{collections::BTreeMap, fmt::Debug};

use bincode::{Decode, Encode};
use sanakirja::Storable;

use crate::{traits::SNAPSHOTS_FOLDER, utils::Binary};

const NUMBER_OF_DAYS_SAVED: i32 = 150;

#[derive(Debug, Encode, Decode)]
pub struct Cache<Key, Value>
where
    Key: Ord + Clone + Debug + Encode + Decode,
    Value: Copy + Storable + PartialEq + Encode + Decode,
{
    date_index: i32,
    path: String,
    map: BTreeMap<Key, (Value, i32)>,
}

impl<Key, Value> Cache<Key, Value>
where
    Key: Ord + Clone + Debug + Encode + Decode,
    Value: Copy + Storable + PartialEq + Encode + Decode,
{
    pub fn import(folder: &str, file: &str) -> color_eyre::Result<Self> {
        let path = format!("{SNAPSHOTS_FOLDER}/{folder}/{file}.bin");

        let map = Binary::import(&path).unwrap_or_default();

        Ok(Self {
            date_index: 0,
            path,
            map,
        })
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.map.get(key).map(|(v, _)| v)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.map.remove(key).map(|(v, _)| v)
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.map
            .insert(key, (value, self.date_index))
            .map(|(v, _)| v)
    }

    pub fn set_date_index(&mut self, date_index: i32) {
        self.date_index = date_index;
    }

    pub fn export(mut self) -> color_eyre::Result<Vec<(Key, Value)>> {
        let mut to_dump = vec![];

        let min_date_index = self.date_index - NUMBER_OF_DAYS_SAVED;

        self.map.retain(|key, tuple| {
            if tuple.1 <= min_date_index {
                to_dump.push((key.clone(), tuple.0));

                false
            } else {
                true
            }
        });

        Binary::export(&self.path, &self.map)?;

        Ok(to_dump)
    }
}
