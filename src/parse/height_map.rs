use std::{cmp::Ordering, fmt::Debug, fs, iter::Sum, ops::Add};

use bincode::{Decode, Encode};
use parking_lot::{lock_api::MutexGuard, RawMutex};
use serde::{de::DeserializeOwned, Serialize};

use crate::{bitcoin::NUMBER_OF_UNSAFE_BLOCKS, io::Serialization};

use super::WMutex;

pub struct HeightMap<T>
where
    T: Clone + Default + Debug + Decode + Encode,
{
    batch: WMutex<Vec<(usize, T)>>,
    path: String,
    initial_last_height: Option<usize>,
    initial_first_unsafe_height: Option<usize>,
    inner: Option<WMutex<Vec<T>>>,
    called_insert: WMutex<bool>,
    serialization: Serialization,
}

impl<T> HeightMap<T>
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
{
    #[allow(unused)]
    pub fn new_on_disk_bin(path: &str) -> Self {
        Self::new(path, false, Serialization::Binary)
    }

    #[allow(unused)]
    pub fn new_in_memory_bin(path: &str) -> Self {
        Self::new(path, true, Serialization::Binary)
    }

    #[allow(unused)]
    pub fn new_on_disk_json(path: &str) -> Self {
        Self::new(path, false, Serialization::Json)
    }

    #[allow(unused)]
    pub fn new_in_memory_json(path: &str) -> Self {
        Self::new(path, true, Serialization::Json)
    }

    fn new(path: &str, in_memory: bool, serialization: Serialization) -> Self {
        fs::create_dir_all(path).unwrap();

        let mut s = Self {
            batch: WMutex::new(vec![]),
            initial_first_unsafe_height: None,
            initial_last_height: None,
            path: serialization.append_extension(&format!("{path}/height")),
            inner: None,
            called_insert: WMutex::new(false),
            serialization,
        };

        if in_memory {
            s.inner.replace(WMutex::new(s.import()));
        }

        s.initial_last_height = s.get_last_height();
        s.initial_first_unsafe_height = last_height_to_first_unsafe_height(s.initial_last_height);

        // dbg!(&s.path, &s.initial_first_unsafe_height);

        s
    }

    pub fn insert(&self, height: usize, value: T) {
        // dbg!(&self.path);

        // We need data to compute datemaps, TODO: change the way date value is computed to avoid recomputing safe height values
        // if !self.is_height_safe(height) {
        *self.called_insert.lock() = true;

        if let Some(list) = self.inner.as_ref() {
            insert_vec(&mut list.lock(), height, value, &self.path);
        } else {
            self.batch.lock().push((height, value));
        }
        // }
    }

    pub fn get_batch(&self) -> MutexGuard<'_, RawMutex, Vec<(usize, T)>> {
        self.batch.lock()
    }

    pub fn insert_default(&self, height: usize) {
        self.insert(height, T::default())
    }

    #[inline(always)]
    pub fn is_height_safe(&self, height: usize) -> bool {
        self.initial_first_unsafe_height.unwrap_or(0) > height
    }

    #[inline(always)]
    pub fn unsafe_inner(&self) -> MutexGuard<'_, RawMutex, Vec<T>> {
        self.inner.as_ref().unwrap().lock()
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn unsafe_len(&self) -> usize {
        self.unsafe_inner().len()
    }

    fn import(&self) -> Vec<T> {
        self.serialization.import(&self.path).unwrap_or_default()
    }

    fn get_last_height(&self) -> Option<usize>
    where
        T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
    {
        let len = self
            .inner
            .as_ref()
            .map(|inner| inner.lock().len())
            .unwrap_or_else(|| self.import().len());

        if len == 0 {
            None
        } else {
            Some(len - 1)
        }
    }
}

impl<T> HeightMap<T>
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned + Add + Sum + Copy,
{
    pub fn sum_last_day_values(&self, from_height: usize) -> T {
        let mut found = false;

        let sum = self
            .get_batch()
            .iter()
            .filter(|(height, _)| {
                if height == &from_height {
                    found = true;
                }

                height >= &from_height
            })
            .map(|(_, value)| *value)
            .sum();

        if !found {
            panic!("Didn't found starting height ({from_height})");
        }

        sum
    }
}

pub trait AnyHeightMap {
    fn get_initial_first_unsafe_height(&self) -> Option<usize>;

    fn get_initial_last_height(&self) -> Option<usize>;

    fn get_last_height(&self) -> Option<usize>;

    fn get_first_unsafe_height(&self) -> Option<usize>;

    fn export(&self) -> color_eyre::Result<()>;

    fn path(&self) -> &str;

    fn t_name(&self) -> &str;

    fn reset(&mut self) -> color_eyre::Result<()>;
}

impl<T> AnyHeightMap for HeightMap<T>
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
{
    #[inline(always)]
    fn get_initial_first_unsafe_height(&self) -> Option<usize> {
        self.initial_first_unsafe_height
    }

    #[inline(always)]
    fn get_initial_last_height(&self) -> Option<usize> {
        self.initial_last_height
    }

    #[inline(always)]
    fn get_last_height(&self) -> Option<usize> {
        self.get_last_height()
    }

    fn get_first_unsafe_height(&self) -> Option<usize> {
        last_height_to_first_unsafe_height(self.get_last_height())
    }

    fn export(&self) -> color_eyre::Result<()> {
        if !self.called_insert.lock().to_owned() {
            return Ok(());
        }

        *self.called_insert.lock() = false;

        if let Some(inner) = self.inner.as_ref() {
            self.serialization.export(&self.path, inner)
        } else {
            if self.batch.lock().is_empty() {
                return Ok(());
            }

            let mut list = self.import();

            self.batch.lock().drain(..).for_each(|(height, value)| {
                insert_vec(&mut list, height, value, &self.path);
            });

            self.serialization.export(&self.path, &list)
        }
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn t_name(&self) -> &str {
        std::any::type_name::<T>()
    }

    fn reset(&mut self) -> color_eyre::Result<()> {
        fs::remove_dir(&self.path)?;

        self.batch.lock().clear();
        self.initial_last_height = None;
        self.initial_first_unsafe_height = None;

        if let Some(vec) = self.inner.as_ref() {
            vec.lock().clear()
        }

        *self.called_insert.lock() = false;

        Ok(())
    }
}

fn insert_vec<T>(list: &mut Vec<T>, height: usize, value: T, path: &str)
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
{
    let height = height.to_owned();
    let value = value.to_owned();
    let len = list.len();

    match height.cmp(&len) {
        Ordering::Greater => {
            panic!(
                "Out of bound push (current len = {}, pushing to = {}, path = {})",
                list.len(),
                height,
                path
            );
        }
        Ordering::Equal => {
            list.push(value);
        }
        Ordering::Less => {
            list[height] = value;
        }
    }
}

fn last_height_to_first_unsafe_height(last_height: Option<usize>) -> Option<usize> {
    last_height.and_then(|last_height| {
        let offset = NUMBER_OF_UNSAFE_BLOCKS - 1;

        if last_height >= offset {
            Some(last_height - offset)
        } else {
            None
        }
    })
}
