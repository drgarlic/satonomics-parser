use std::fmt::Debug;

use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use super::{DateMap, HeightMap};

pub struct BiMap<T>
where
    T: Clone + Default + Debug + Decode + Encode,
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
