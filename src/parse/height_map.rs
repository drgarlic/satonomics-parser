use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs,
    iter::Sum,
    mem,
    ops::{Add, AddAssign, DerefMut, Div, Mul, RangeInclusive, Sub, SubAssign},
    path::{Path, PathBuf},
};

use itertools::Itertools;
use ordered_float::{FloatCore, OrderedFloat};
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    bitcoin::{BLOCKS_PER_HAVLING_EPOCH, NUMBER_OF_UNSAFE_BLOCKS},
    io::{format_path, Serialization},
    utils::ToF32,
};

use super::AnyMap;

const CHUNK_SIZE: usize = BLOCKS_PER_HAVLING_EPOCH / 8;

pub struct HeightMap<T>
where
    T: Clone + Default + Debug + savefile::Serialize + savefile::Deserialize,
{
    path_all: String,
    path_last: Option<String>,

    serialization: Serialization,

    initial_last_height: Option<usize>,
    initial_first_unsafe_height: Option<usize>,

    imported: RwLock<BTreeMap<usize, BTreeMap<usize, T>>>,
    to_insert: RwLock<BTreeMap<usize, BTreeMap<usize, T>>>,
}

impl<T> HeightMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
    #[allow(unused)]
    pub fn new_bin(path: &str) -> Self {
        Self::new(path, Serialization::Binary, true)
    }

    #[allow(unused)]
    pub fn _new_bin(path: &str, export_last: bool) -> Self {
        Self::new(path, Serialization::Binary, export_last)
    }

    #[allow(unused)]
    pub fn new_json(path: &str) -> Self {
        Self::new(path, Serialization::Json, true)
    }

    #[allow(unused)]
    pub fn _new_json(path: &str, export_last: bool) -> Self {
        Self::new(path, Serialization::Json, export_last)
    }

    fn new(path: &str, serialization: Serialization, export_last: bool) -> Self {
        let path = format_path(path);

        let path_all = format!("{path}/height");

        fs::create_dir_all(&path_all).unwrap();

        let path_last = {
            if export_last {
                Some(serialization.append_extension(&format!("{path}/last")))
            } else {
                None
            }
        };

        let mut s = Self {
            path_all,
            path_last,

            serialization,

            initial_first_unsafe_height: None,
            initial_last_height: None,

            to_insert: RwLock::new(BTreeMap::default()),
            imported: RwLock::new(BTreeMap::default()),
        };

        s.import_last();

        s.initial_last_height = s
            .imported
            .read()
            .values()
            .last()
            .and_then(|d| d.keys().cloned().max());

        s.initial_first_unsafe_height = s.initial_last_height.and_then(|last_height| {
            let offset = NUMBER_OF_UNSAFE_BLOCKS - 1;
            last_height.checked_sub(offset)
        });

        s
    }

    fn height_to_chunk_name(height: usize) -> String {
        let start = Self::height_to_chunk_start(height);
        let end = start + CHUNK_SIZE;

        format!("{start}..{end}")
    }

    fn height_to_chunk_start(height: usize) -> usize {
        height / CHUNK_SIZE * CHUNK_SIZE
    }

    pub fn insert(&self, height: usize, value: T) {
        if !self.is_height_safe(height) {
            self.to_insert
                .write()
                .entry(Self::height_to_chunk_start(height))
                .or_default()
                .insert(height, value);
        } else {
            panic!("Shouldn't have called insert")
        }
    }

    pub fn insert_default(&self, height: usize) {
        self.insert(height, T::default())
    }

    pub fn get(&self, height: &usize) -> Option<T> {
        let chunk_start = Self::height_to_chunk_start(*height);

        self.to_insert
            .read()
            .get(&chunk_start)
            .and_then(|tree| tree.get(height).cloned())
            .or_else(|| {
                self.import_chunk_if_needed(*height);

                self.imported
                    .read()
                    .get(&chunk_start)
                    .and_then(|tree| tree.get(height))
                    .cloned()
            })
    }

    pub fn sum(&self, range: RangeInclusive<usize>) -> T
    where
        T: Sum,
    {
        range.flat_map(|height| self.get(&height)).sum::<T>()
    }

    #[inline(always)]
    pub fn is_height_safe(&self, height: usize) -> bool {
        self.initial_first_unsafe_height.unwrap_or(0) > height
    }

    fn read_dir(&self) -> BTreeMap<usize, PathBuf> {
        fs::read_dir(&self.path_all)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                let extension = path.extension().unwrap().to_str().unwrap();

                path.is_file() && extension == self.serialization.to_str()
            })
            .map(|path| {
                (
                    path.file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .split("..")
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                    path,
                )
            })
            .collect()
    }

    fn import_chunk_if_needed(&self, height: usize) {
        let chunk_start = Self::height_to_chunk_start(height);

        if let Some(path) = self.read_dir().get(&chunk_start) {
            if self.imported.read().get(&chunk_start).is_none() {
                if let Ok(map) = self._import(path) {
                    self.imported.write().insert(chunk_start, map);
                }
            }
        }
    }

    fn import_last(&self) {
        if let Some((chunk_start, path)) = self.read_dir().into_iter().last() {
            if let Ok(map) = self._import(&path) {
                self.imported.write().insert(chunk_start, map);
            }
        }
    }

    fn _import(&self, path: &Path) -> color_eyre::Result<BTreeMap<usize, T>> {
        let chunk_start = path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .split("..")
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(self
            .serialization
            .import::<Vec<T>>(path.to_str().unwrap())?
            .into_iter()
            .enumerate()
            .map(|(index, value)| (chunk_start + index, value))
            .collect())
    }
}

impl<T> AnyMap for HeightMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
    // fn import_tmp_data(&self) {
    //     // println!("import tmp {}", &self.path);

    //     if !self.modified.read().to_owned() {
    //         return;
    //     }

    //     if self.storage == Storage::Disk {
    //         if self.imported.read().is_some() {
    //             dbg!(&self.path);
    //             panic!("Probably forgot to drop inner after an export");
    //         }

    //         self.import_to_inner();

    //         self.to_insert.read().drain(..).for_each(|(height, value)| {
    //             self.insert_to_inner(height, value);
    //         });
    //     }
    // }

    fn path(&self) -> &str {
        &self.path_all
    }

    fn t_name(&self) -> &str {
        std::any::type_name::<T>()
    }

    fn reset(&mut self) -> color_eyre::Result<()> {
        fs::remove_dir(&self.path_all)?;

        self.initial_last_height = None;
        self.initial_first_unsafe_height = None;

        self.imported.write().clear();
        self.to_insert.write().clear();

        Ok(())
    }

    fn export(&self) -> color_eyre::Result<()> {
        let to_insert = mem::take(self.to_insert.write().deref_mut());

        match to_insert.iter().next() {
            Some(first_map_to_insert) => {
                let first_height_to_insert = first_map_to_insert.1.iter().next().unwrap().0;

                if first_height_to_insert % CHUNK_SIZE != 0 {
                    self.import_chunk_if_needed(*first_height_to_insert);
                }
            }
            None => return Ok(()),
        }

        let mut imported = self.imported.write();

        let len = imported.len();

        to_insert.into_iter().enumerate().try_for_each(
            |(index, (chunk_start, map))| -> color_eyre::Result<()> {
                let chunk_name = Self::height_to_chunk_name(chunk_start);

                let to_export = imported.entry(chunk_start.to_owned()).or_default();

                to_export.extend(map);

                let path = self
                    .serialization
                    .append_extension(&format!("{}/{}", self.path_all, chunk_name));

                self.serialization.export(&path, to_export)?;

                if index == len - 1 {
                    if let Some(path_last) = self.path_last.as_ref() {
                        self.serialization
                            .export(path_last, to_export.values().last().unwrap())?;
                    }
                }

                Ok(())
            },
        )
    }

    fn clean(&self) {
        let mut imported = self.imported.write();

        let len = imported.len();

        let keys = imported.keys().cloned().collect_vec();

        keys.into_iter()
            .enumerate()
            .filter(|(index, _)| index + 1 < len)
            .for_each(|(_, key)| {
                imported.remove(&key);
            });
    }
}

pub trait AnyHeightMap: AnyMap {
    fn get_initial_first_unsafe_height(&self) -> Option<usize>;

    fn get_initial_last_height(&self) -> Option<usize>;

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync);
}

impl<T> AnyHeightMap for HeightMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Send
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC
        + Sync,
{
    #[inline(always)]
    fn get_initial_first_unsafe_height(&self) -> Option<usize> {
        self.initial_first_unsafe_height
    }

    #[inline(always)]
    fn get_initial_last_height(&self) -> Option<usize> {
        self.initial_last_height
    }

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync) {
        self
    }
}

impl<T> HeightMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
    // pub fn transform<F>(&self, transform: F) -> Vec<T>
    // where
    //     T: Copy + Default,
    //     F: Fn((usize, &T, &[T])) -> T,
    // {
    //     Self::_transform(self.inner.read().as_ref().unwrap(), transform)
    // }

    #[allow(unused)]
    pub fn _transform<F>(vec: &[T], transform: F) -> Vec<T>
    where
        T: Copy + Default,
        F: Fn((usize, &T, &[T])) -> T,
    {
        vec.iter()
            .enumerate()
            .map(|(index, value)| transform((index, value, &vec)))
            .collect_vec()
    }

    // #[allow(unused)]
    // pub fn add(&self, other: &Self) -> Vec<T>
    // where
    //     T: Add<Output = T> + Copy + Default,
    // {
    //     Self::_add(
    //         self.inner.read().as_ref().unwrap(),
    //         other.inner.read().as_ref().unwrap(),
    //     )
    // }

    #[allow(unused)]
    pub fn _add(arr1: &[T], arr2: &[T]) -> Vec<T>
    where
        T: Add<Output = T> + Copy + Default,
    {
        Self::_transform(Self::slice_arr1(arr1, arr2), |(index, value, _)| {
            *value + *arr2.get(index).unwrap()
        })
    }

    // #[allow(unused)]
    // pub fn subtract(&self, other: &Self) -> Vec<T>
    // where
    //     T: Sub<Output = T> + Copy + Default,
    // {
    //     Self::_subtract(
    //         self.inner.read().as_ref().unwrap(),
    //         other.inner.read().as_ref().unwrap(),
    //     )
    // }

    #[allow(unused)]
    pub fn _subtract(arr1: &[T], arr2: &[T]) -> Vec<T>
    where
        T: Sub<Output = T> + Copy + Default,
    {
        Self::_transform(Self::slice_arr1(arr1, arr2), |(index, value, _)| {
            *value - *arr2.get(index).unwrap()
        })
    }

    // #[allow(unused)]
    // pub fn multiply(&self, other: &Self) -> Vec<T>
    // where
    //     T: Mul<Output = T> + Copy + Default,
    // {
    //     Self::_multiply(
    //         self.inner.read().as_ref().unwrap(),
    //         other.inner.read().as_ref().unwrap(),
    //     )
    // }

    #[allow(unused)]
    pub fn _multiply(arr1: &[T], arr2: &[T]) -> Vec<T>
    where
        T: Mul<Output = T> + Copy + Default,
    {
        Self::_transform(Self::slice_arr1(arr1, arr2), |(index, value, _)| {
            *value * *arr2.get(index).unwrap()
        })
    }

    // #[allow(unused)]
    // pub fn divide(&self, other: &Self) -> Vec<T>
    // where
    //     T: Div<Output = T> + Copy + Default,
    // {
    //     Self::_divide(
    //         self.inner.read().as_ref().unwrap(),
    //         other.inner.read().as_ref().unwrap(),
    //     )
    // }

    #[allow(unused)]
    pub fn _divide(arr1: &[T], arr2: &[T]) -> Vec<T>
    where
        T: Div<Output = T> + Copy + Default,
    {
        Self::_transform(Self::slice_arr1(arr1, arr2), |(index, value, _)| {
            *value / *arr2.get(index).unwrap()
        })
    }

    #[allow(unused)]
    fn slice_arr1<'a>(arr1: &'a [T], arr2: &'a [T]) -> &'a [T] {
        if arr1.len() > arr2.len() {
            &arr1[..arr2.len()]
        } else {
            arr1
        }
    }

    // pub fn cumulate(&self) -> Vec<T>
    // where
    //     T: Sum + Copy + Default + AddAssign,
    // {
    //     Self::_cumulate(self.inner.read().as_ref().unwrap())
    // }

    #[allow(unused)]
    pub fn _cumulate(arr: &[T]) -> Vec<T>
    where
        T: Sum + Copy + Default + AddAssign,
    {
        let mut sum = T::default();

        arr.iter()
            .map(|value| {
                sum += *value;
                sum
            })
            .collect_vec()
    }

    // #[allow(unused)]
    // pub fn last_x_sum(&self, x: usize) -> Vec<T>
    // where
    //     T: Sum + Copy + Default + AddAssign + SubAssign,
    // {
    //     Self::_last_x_sum(self.inner.read().as_ref().unwrap(), x)
    // }

    #[allow(unused)]
    pub fn _last_x_sum(arr: &[T], x: usize) -> Vec<T>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign,
    {
        let mut sum = T::default();

        arr.iter()
            .enumerate()
            .map(|(index, value)| {
                sum += *value;

                if index >= x - 1 {
                    let previous_index = index + 1 - x;

                    sum -= *arr.get(previous_index).unwrap()
                }

                sum
            })
            .collect_vec()
    }

    // #[allow(unused)]
    // pub fn moving_average(&self, x: usize) -> Vec<f32>
    // where
    //     T: Sum + Copy + Default + AddAssign + SubAssign + ToF32,
    // {
    //     Self::_moving_average(self.inner.read().as_ref().unwrap(), x)
    // }

    #[allow(unused)]
    pub fn _moving_average(arr: &[T], x: usize) -> Vec<f32>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign + ToF32,
    {
        let mut sum = T::default();

        arr.iter()
            .enumerate()
            .map(|(index, value)| {
                sum += *value;

                if index >= x - 1 {
                    sum -= *arr.get(index + 1 - x).unwrap()
                }

                sum.to_f32() / x as f32
            })
            .collect_vec()
    }

    // #[allow(unused)]
    // pub fn net_change(&self, offset: usize) -> Vec<T>
    // where
    //     T: Copy + Default + Sub<Output = T>,
    // {
    //     Self::_net_change(self.inner.read().as_ref().unwrap(), offset)
    // }

    #[allow(unused)]
    pub fn _net_change(arr: &[T], offset: usize) -> Vec<T>
    where
        T: Copy + Default + Sub<Output = T>,
    {
        Self::_transform(arr, |(index, value, arr)| {
            let previous = {
                if let Some(previous_index) = index.checked_sub(offset) {
                    *arr.get(previous_index).unwrap()
                } else {
                    T::default()
                }
            };

            *value - previous
        })
    }

    // #[allow(unused)]
    // pub fn median(&self, size: usize) -> Vec<Option<T>>
    // where
    //     T: FloatCore,
    // {
    //     Self::_median(self.inner.read().as_ref().unwrap(), size)
    // }

    #[allow(unused)]
    pub fn _median(arr: &[T], size: usize) -> Vec<Option<T>>
    where
        T: FloatCore,
    {
        let even = size % 2 == 0;
        let median_index = size / 2;

        if size < 3 {
            panic!("Computing a median for a size lower than 3 is useless");
        }

        arr.iter()
            .enumerate()
            .map(|(index, _)| {
                if index >= size - 1 {
                    let mut arr = arr[index - (size - 1)..index + 1]
                        .iter()
                        .map(|value| OrderedFloat(*value))
                        .collect_vec();

                    arr.sort_unstable();

                    if even {
                        Some(
                            (**arr.get(median_index).unwrap()
                                + **arr.get(median_index - 1).unwrap())
                                / T::from(2.0).unwrap(),
                        )
                    } else {
                        Some(**arr.get(median_index).unwrap())
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
