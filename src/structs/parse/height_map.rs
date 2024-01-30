use std::{
    cmp::Ordering,
    fmt::Debug,
    fs,
    sync::{RwLock, RwLockReadGuard},
};

use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use crate::{bitcoin::NUMBER_OF_UNSAFE_BLOCKS, structs::Serialization};

pub struct HeightMap<T>
where
    T: Clone + Default + Debug + Decode + Encode,
{
    batch: RwLock<Vec<(usize, T)>>,
    path: String,
    initial_first_unsafe_height: Option<usize>,
    inner: Option<RwLock<Vec<T>>>,
    called_insert: RwLock<bool>,
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
            batch: RwLock::new(vec![]),
            initial_first_unsafe_height: None,
            path: serialization.append_extension(&format!("{path}/height")),
            inner: None,
            called_insert: RwLock::new(false),
            serialization,
        };

        if in_memory {
            s.inner.replace(RwLock::new(s.import()));
        }

        s.initial_first_unsafe_height = s.get_first_unsafe_height();

        s
    }

    pub fn insert(&self, height: usize, value: T) {
        if !self.is_height_safe(height) {
            *self.called_insert.write().unwrap() = true;

            if let Some(list) = self.inner.as_ref() {
                insert_vec(&mut list.write().unwrap(), height, value, &self.path);
            } else {
                self.batch.write().unwrap().push((height, value));
            }
        }
    }

    pub fn insert_default(&self, height: usize) {
        self.insert(height, T::default())
    }

    pub fn is_height_safe(&self, height: usize) -> bool {
        self.initial_first_unsafe_height.unwrap_or(0) > height
    }

    pub fn unsafe_inner(&self) -> RwLockReadGuard<'_, Vec<T>> {
        self.inner.as_ref().unwrap().read().unwrap()
    }

    fn import(&self) -> Vec<T> {
        self.serialization.import(&self.path).unwrap_or_default()
    }

    fn get_first_unsafe_height(&self) -> Option<usize>
    where
        T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
    {
        self.get_last_height().and_then(|last_height| {
            let offset = NUMBER_OF_UNSAFE_BLOCKS - 1;

            if last_height >= offset {
                Some(last_height - offset)
            } else {
                None
            }
        })
    }

    fn get_last_height(&self) -> Option<usize>
    where
        T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
    {
        let len = self
            .inner
            .as_ref()
            .map(|inner| inner.read().unwrap().len())
            .unwrap_or_else(|| self.import().len());

        if len == 0 {
            None
        } else {
            Some(len - 1)
        }
    }
}

pub trait AnyHeightMap {
    fn get_initial_first_unsafe_height(&self) -> Option<usize>;

    fn get_last_height(&self) -> Option<usize>;

    fn get_first_unsafe_height(&self) -> Option<usize>;

    fn export(&self) -> color_eyre::Result<()>;
}

impl<T> AnyHeightMap for HeightMap<T>
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned,
{
    fn get_initial_first_unsafe_height(&self) -> Option<usize> {
        self.initial_first_unsafe_height
    }

    fn get_last_height(&self) -> Option<usize> {
        self.get_last_height()
    }

    fn get_first_unsafe_height(&self) -> Option<usize> {
        self.get_first_unsafe_height()
    }

    fn export(&self) -> color_eyre::Result<()> {
        if !self.called_insert.read().unwrap().to_owned() {
            return Ok(());
        }

        *self.called_insert.write().unwrap() = false;

        if let Some(inner) = self.inner.as_ref() {
            self.serialization.export(&self.path, inner)
        } else {
            if self.batch.read().unwrap().is_empty() {
                return Ok(());
            }

            let mut list = self.import();

            self.batch
                .write()
                .unwrap()
                .drain(..)
                .for_each(|(height, value)| {
                    insert_vec(&mut list, height, value, &self.path);
                });

            self.serialization.export(&self.path, &list)
        }
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
