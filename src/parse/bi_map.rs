use std::{fmt::Debug, iter::Sum, ops::RangeInclusive};

use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Serialize};

use super::{AnyDateMap, AnyHeightMap, AnyMap, DateMap, HeightMap};

pub struct BiMap<T>
where
    T: Clone
        + Copy
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
        + Copy
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
    pub fn new_bin(version: u32, path: &str) -> Self {
        Self {
            height: HeightMap::_new_bin(version, path, 1, true),
            date: DateMap::_new_bin(version, path, 1, false),
        }
    }

    pub fn _new_bin(version: u32, path: &str, height_chunks_in_memory: usize) -> Self {
        Self {
            height: HeightMap::_new_bin(version, path, height_chunks_in_memory, true),
            date: DateMap::_new_bin(version, path, height_chunks_in_memory, false),
        }
    }

    // pub fn new_json(path: &str) -> Self {
    //     Self {
    //         height: HeightMap::_new_json(path, true),
    //         date: DateMap::_new_json(path, false),
    //     }
    // }

    pub fn date_insert_sum_range(
        &mut self,
        date: NaiveDate,
        date_blocks_range: &RangeInclusive<usize>,
    ) {
        self.date
            .insert(date, self.height.sum_range(date_blocks_range));
    }
}

pub trait AnyBiMap {
    fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool;

    fn as_any_map(&self) -> Vec<&(dyn AnyMap + Send + Sync)>;

    fn as_any_mut_map(&mut self) -> Vec<&mut dyn AnyMap>;

    fn get_height(&self) -> &(dyn AnyHeightMap + Send + Sync);

    fn get_date(&self) -> &(dyn AnyDateMap + Send + Sync);
}

impl<T> AnyBiMap for BiMap<T>
where
    T: Clone
        + Copy
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

    fn as_any_mut_map(&mut self) -> Vec<&mut dyn AnyMap> {
        vec![self.date.as_any_mut_map(), self.height.as_any_mut_map()]
    }

    fn get_height(&self) -> &(dyn AnyHeightMap + Send + Sync) {
        &self.height
    }

    fn get_date(&self) -> &(dyn AnyDateMap + Send + Sync) {
        &self.date
    }
}
