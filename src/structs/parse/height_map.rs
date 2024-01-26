use std::{cmp::Ordering, fmt::Debug, path::Path, sync::RwLock};

use serde::{de::DeserializeOwned, Serialize};

use crate::{bitcoin::NUMBER_OF_UNSAFE_BLOCKS, structs::Json};

pub struct HeightMap<T>
where
    T: Clone,
{
    batch: RwLock<Vec<(usize, T)>>,
    path: String,
    initial_first_unsafe_height: Option<usize>,
}

impl<T> HeightMap<T>
where
    T: Clone + DeserializeOwned + Serialize + Default + Debug,
{
    pub fn new(path: &str) -> Self {
        Self {
            batch: RwLock::new(vec![]),
            initial_first_unsafe_height: get_first_unsafe_height::<T, _>(&path),
            path: path.to_string(),
        }
    }

    pub fn insert(&self, height: usize, value: T) {
        if self.initial_first_unsafe_height.unwrap_or(0) <= height {
            self.batch.write().unwrap().push((height, value));
        }
    }

    pub fn insert_default(&self, height: usize) {
        self.insert(height, T::default())
    }

    pub fn consume(self) -> Vec<T> {
        self.import()
    }

    fn import(&self) -> Vec<T> {
        Json::import_vec(&self.path)
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
    T: Clone + DeserializeOwned + Serialize + Default + Debug,
{
    fn get_initial_first_unsafe_height(&self) -> Option<usize> {
        self.initial_first_unsafe_height
    }

    fn get_last_height(&self) -> Option<usize> {
        get_last_height::<T, _>(&self.path)
    }

    fn get_first_unsafe_height(&self) -> Option<usize> {
        get_first_unsafe_height::<T, _>(&self.path)
    }

    fn export(&self) -> color_eyre::Result<()> {
        let len = self.batch.read().unwrap().len();

        if len == 0 {
            return Ok(());
        }

        let mut list = self.import();

        self.batch
            .write()
            .unwrap()
            .drain(..)
            .for_each(|(height, value)| {
                let height = height.to_owned();
                let value = value.to_owned();
                let len = list.len();

                match height.cmp(&len) {
                    Ordering::Greater => {
                        panic!(
                            "Out of bound push (current len = {}, pushing to = {}, path = {})",
                            list.len(),
                            height,
                            self.path,
                        );
                    }
                    Ordering::Equal => {
                        list.push(value);
                    }
                    Ordering::Less => {
                        list[height] = value;
                    }
                }
            });

        Json::export(&self.path, &list)
    }
}

fn get_first_unsafe_height<T, P>(path: P) -> Option<usize>
where
    T: Clone + DeserializeOwned + Serialize,
    P: AsRef<Path>,
{
    get_last_height::<T, P>(path).and_then(|last_height| {
        let offset = NUMBER_OF_UNSAFE_BLOCKS - 1;

        if last_height >= offset {
            Some(last_height - offset)
        } else {
            None
        }
    })
}

fn get_last_height<T, P>(path: P) -> Option<usize>
where
    T: Clone + DeserializeOwned + Serialize,
    P: AsRef<Path>,
{
    let len = Json::import_vec::<T, P>(path).len();

    if len == 0 {
        None
    } else {
        Some(len - 1)
    }
}
