use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
    sync::RwLock,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::utils::{export_pretty_json, import_json_vec, EXPORTS_FOLDER_RAW_PATH};

pub const NUMBER_OF_UNSAFE_BLOCKS: usize = 100;

pub struct HeightMap<T>
where
    T: Clone,
{
    batch: RwLock<Vec<(usize, T)>>,
    path: PathBuf,
    initial_first_unsafe_height: Option<usize>,
}

impl<T> HeightMap<T>
where
    T: Clone + DeserializeOwned + Serialize,
{
    pub fn new(path: &str) -> Self {
        let path_buf = Path::new(EXPORTS_FOLDER_RAW_PATH).join(path);

        Self {
            batch: RwLock::new(vec![]),
            initial_first_unsafe_height: get_first_unsafe_height::<T>(&path_buf),
            path: path_buf,
        }
    }

    pub fn insert(&self, height: usize, value: T) {
        if self.initial_first_unsafe_height.unwrap_or(0) <= height {
            self.batch.write().unwrap().push((height, value));
        }
    }

    pub fn consume(self) -> Vec<T> {
        self.import().expect("import in consume to work")
    }

    fn import(&self) -> color_eyre::Result<Vec<T>> {
        import::<T>(&self.path)
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
    T: Clone + DeserializeOwned + Serialize,
{
    fn get_initial_first_unsafe_height(&self) -> Option<usize> {
        self.initial_first_unsafe_height
    }

    fn get_last_height(&self) -> Option<usize> {
        get_last_height::<T>(&self.path)
    }

    fn get_first_unsafe_height(&self) -> Option<usize> {
        get_first_unsafe_height::<T>(&self.path)
    }

    fn export(&self) -> color_eyre::Result<()> {
        let len = self.batch.read().unwrap().len();

        if len == 0 {
            return Ok(());
        }

        let mut list = self.import()?;

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
                            "Out of bound push (current len = {}, pushing to = {})",
                            list.len(),
                            height
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

        export_pretty_json(&self.path, &list)
    }
}

fn import<T>(path: &Path) -> color_eyre::Result<Vec<T>>
where
    T: Clone + DeserializeOwned + Serialize,
{
    import_json_vec::<T>(path, true)
}

fn get_last_height<T>(path: &Path) -> Option<usize>
where
    T: Clone + DeserializeOwned + Serialize,
{
    let len = import::<T>(path).expect("get last height to work").len();

    if len == 0 {
        None
    } else {
        Some(len - 1)
    }
}

fn get_first_unsafe_height<T>(path: &Path) -> Option<usize>
where
    T: Clone + DeserializeOwned + Serialize,
{
    get_last_height::<T>(path).and_then(|last_height| {
        let offset = NUMBER_OF_UNSAFE_BLOCKS - 1;

        if last_height >= offset {
            Some(last_height - offset)
        } else {
            None
        }
    })
}
