use std::{fmt::Debug, iter::Sum};

use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Serialize};

use super::{AnyDateMap, AnyHeightMap, AnyMap, DateMap, HeightMap};

pub struct BiMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
    pub height: HeightMap<T>,
    pub date: DateMap<T>,
}

impl<T> BiMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
    pub fn new_bin(path: &str) -> Self {
        Self {
            height: HeightMap::_new_bin(path, true),
            date: DateMap::_new_bin(path, false),
        }
    }

    #[allow(unused)]
    pub fn new_json(path: &str) -> Self {
        Self {
            height: HeightMap::_new_json(path, true),
            date: DateMap::_new_json(path, false),
        }
    }
}

pub trait AnyBiMap {
    fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool;

    fn as_any_map(&self) -> Vec<&(dyn AnyMap + Send + Sync)>;
}

impl<T> AnyBiMap for BiMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC
        + Send
        + Sync,
{
    #[inline(always)]
    fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool {
        self.date.is_date_safe(date) && self.height.is_height_safe(height)
    }

    fn as_any_map(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        vec![self.date.as_any_map(), self.height.as_any_map()]
    }
}
