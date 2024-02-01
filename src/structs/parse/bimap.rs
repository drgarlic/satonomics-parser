use std::fmt::Debug;

use bincode::{Decode, Encode};
use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Serialize};

use super::{DateMap, HeightMap};

pub struct BiMap<T>
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
{
    pub height: HeightMap<T>,
    pub date: DateMap<T>,
}

impl<T> BiMap<T>
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
{
    #[allow(unused)]
    pub fn new_on_disk_bin(path: &str) -> Self {
        Self {
            height: HeightMap::new_on_disk_bin(path),
            date: DateMap::new_on_disk_bin(path),
        }
    }

    #[allow(unused)]
    pub fn new_in_memory_bin(path: &str) -> Self {
        Self {
            height: HeightMap::new_in_memory_bin(path),
            date: DateMap::new_in_memory_bin(path),
        }
    }

    #[allow(unused)]
    pub fn new_on_disk_json(path: &str) -> Self {
        Self {
            height: HeightMap::new_on_disk_json(path),
            date: DateMap::new_on_disk_json(path),
        }
    }

    #[allow(unused)]
    pub fn new_in_memory_json(path: &str) -> Self {
        Self {
            height: HeightMap::new_in_memory_json(path),
            date: DateMap::new_in_memory_json(path),
        }
    }
}

pub trait AnyBiMap {
    fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool;
}

impl<T> AnyBiMap for BiMap<T>
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
{
    fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool {
        self.date.is_date_safe(date) && self.height.is_height_safe(height)
    }
}
