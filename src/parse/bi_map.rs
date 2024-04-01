use std::{collections::BTreeMap, fmt::Debug, iter::Sum};

use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Serialize};

use super::{AnyExportableMap, DateMap, HeightMap, HeightToDateConverter, WNaiveDate};

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

    pub fn set_height(&self, vec: Vec<T>) {
        self.height.set_inner(vec)
    }

    pub fn set_date(&self, map: BTreeMap<WNaiveDate, T>) {
        self.date.set_inner(map)
    }

    pub fn set_height_then_compute_date(&self, vec: Vec<T>, converter: &HeightToDateConverter) {
        self.set_height(vec);
        self.compute_date(converter)
    }

    pub fn compute_date(&self, converter: &HeightToDateConverter) {
        self.date
            .compute_from_height_map(self.height.inner.lock().as_ref().unwrap(), converter);
    }
}

pub trait AnyBiMap {
    fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool;
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
        + savefile::ReprC,
{
    #[inline(always)]
    fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool {
        self.date.is_date_safe(date) && self.height.is_height_safe(height)
    }
}

impl<T> AnyExportableMap for BiMap<T>
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
    fn export_then_clean(&self) -> color_eyre::Result<()> {
        self.height.export_then_clean()?;

        self.date.export_then_clean()?;

        Ok(())
    }
}
