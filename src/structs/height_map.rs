use std::{
    cell::RefCell,
    cmp::Ordering,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::utils::{export_json, import_json_vec, OUTPUTS_FOLDER_RAW_PATH};

pub const NUMBER_OF_UNSAFE_BLOCKS: usize = 100;

pub struct HeightMap<T>
where
    T: Clone,
{
    batch: RefCell<Vec<(usize, T)>>,
    path: PathBuf,
}

impl<T> HeightMap<T>
where
    T: Clone + DeserializeOwned + Serialize,
{
    pub fn new(path: &str) -> Self {
        let path = Path::new(OUTPUTS_FOLDER_RAW_PATH).join(path);

        Self {
            batch: RefCell::new(vec![]),
            path: path.to_owned(),
        }
    }

    pub fn get_last_height(&self) -> Option<usize> {
        let len = self.import().expect("get last height to work").len();

        if len == 0 {
            None
        } else {
            Some(len - 1)
        }
    }

    // pub fn get_last_height_str(&self) -> Option<String> {
    //     self.get_last_height().map(|i| i.to_string())
    // }

    // pub fn get_last_value(&self) -> Option<T> {
    //     self.get_last_height_str()
    //         .as_ref()
    //         .map(|key| self.map.borrow().get(key).cloned().unwrap())
    // }

    pub fn get_first_unsafe_height(&self) -> Option<usize> {
        self.get_last_height().and_then(|last_height| {
            let offset = NUMBER_OF_UNSAFE_BLOCKS - 1;

            if last_height >= offset {
                Some(last_height - offset)
            } else {
                None
            }
        })
    }

    // pub fn get_last_safe_value(&self) -> Option<T> {
    //     self.get_first_unsafe_height().and_then(|index| {
    //         if index > 0 {
    //             Some(
    //                 self.map
    //                     .borrow()
    //                     .get(&(index - 1).to_string())
    //                     .cloned()
    //                     .unwrap(),
    //             )
    //         } else {
    //             None
    //         }
    //     })
    // }

    pub fn insert(&self, height: usize, value: T) {
        self.batch.borrow_mut().push((height, value));

        let len = self.batch.borrow().len();

        if len != 0 && len % 1_000 == 0 {
            self.export().expect("JSON export to work");
        }
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        let mut list = self.import()?;

        self.batch
            .borrow_mut()
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

        export_json(&self.path, &list, true)
    }

    pub fn consume(self) -> Vec<T> {
        self.import().expect("import in consume to work")
    }

    fn import(&self) -> color_eyre::Result<Vec<T>> {
        import_json_vec::<T>(&self.path, true)
    }
}
