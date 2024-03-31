use std::{
    cmp::Ordering,
    fmt::Debug,
    fs,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use itertools::Itertools;
use ordered_float::{FloatCore, OrderedFloat};
use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    bitcoin::NUMBER_OF_UNSAFE_BLOCKS,
    io::{format_path, Serialization},
    utils::ToF32,
};

use super::{AnyExportableMap, AnyMap, Storage};

pub struct HeightMap<T>
where
    T: Clone + Default + Debug + savefile::Serialize + savefile::Deserialize,
{
    storage: Storage,

    path: String,

    batch: Mutex<Vec<(usize, T)>>,

    initial_last_height: Option<usize>,
    initial_first_unsafe_height: Option<usize>,

    pub inner: Mutex<Option<Vec<T>>>,

    modified: Mutex<bool>,

    serialization: Serialization,
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
    pub fn new_on_disk_bin(path: &str) -> Self {
        Self::new(path, Storage::Disk, Serialization::Binary)
    }

    #[allow(unused)]
    pub fn new_in_memory_bin(path: &str) -> Self {
        Self::new(path, Storage::Memory, Serialization::Binary)
    }

    #[allow(unused)]
    pub fn new_on_disk_json(path: &str) -> Self {
        Self::new(path, Storage::Disk, Serialization::Json)
    }

    #[allow(unused)]
    pub fn new_in_memory_json(path: &str) -> Self {
        Self::new(path, Storage::Memory, Serialization::Json)
    }

    fn new(path: &str, storage: Storage, serialization: Serialization) -> Self {
        let path = format_path(path);

        fs::create_dir_all(&path).unwrap();

        let mut s = Self {
            storage: storage.to_owned(),
            batch: Mutex::new(vec![]),
            initial_first_unsafe_height: None,
            initial_last_height: None,
            path: serialization.append_extension(&format!("{path}/height")),
            inner: Mutex::new(None),
            modified: Mutex::new(false),
            serialization,
        };

        if Storage::Memory == storage {
            s.import_to_inner();
        }

        s.initial_last_height = s.get_last_height();
        s.initial_first_unsafe_height = last_height_to_first_unsafe_height(s.initial_last_height);

        s
    }

    pub fn set_inner(&self, vec: Vec<T>) {
        *self.modified.lock() = true;

        self.inner.lock().replace(vec);
    }

    fn import_to_inner(&self) {
        self.set_inner(self.import());
    }

    pub fn insert(&self, height: usize, value: T) {
        *self.modified.lock() = true;

        if self.inner.lock().is_some() {
            self.insert_to_inner(height, value);
        } else {
            self.batch.lock().push((height, value));
        }
    }

    pub fn insert_default(&self, height: usize) {
        self.insert(height, T::default())
    }

    #[inline(always)]
    pub fn is_height_safe(&self, height: usize) -> bool {
        self.initial_first_unsafe_height.unwrap_or(0) > height
    }

    fn import(&self) -> Vec<T> {
        self.serialization.import(&self.path).unwrap_or_default()
    }

    fn get_last_height(&self) -> Option<usize>
    where
        T: Clone + Default + Debug + Serialize + DeserializeOwned,
    {
        let len = self
            .inner
            .lock()
            .as_ref()
            .map(|inner| inner.len())
            .unwrap_or_else(|| self.import().len());

        if len == 0 {
            None
        } else {
            Some(len - 1)
        }
    }

    fn export(&self) -> color_eyre::Result<()> {
        let mut modified = self.modified.lock();

        if !*modified {
            return Ok(());
        }

        *modified = false;

        self.serialization
            .export(&self.path, self.inner.lock().as_ref().unwrap())
    }

    fn clean_tmp_data(&self) {
        if self.storage == Storage::Disk {
            self.inner.lock().take();
        }
    }

    fn insert_to_inner(&self, height: usize, value: T) {
        let mut inner = self.inner.lock();

        let list = inner.as_mut().unwrap();

        let len = list.len();

        match height.cmp(&len) {
            Ordering::Equal => {
                list.push(value);
            }
            Ordering::Less => {
                list[height] = value;
            }
            Ordering::Greater => {
                panic!(
                    "Out of bound push (current len = {}, pushing to = {}, path = {})",
                    list.len(),
                    height,
                    self.path
                );
            }
        }
    }
}

pub trait AnyHeightMap: AnyMap {
    fn get_initial_first_unsafe_height(&self) -> Option<usize>;

    fn get_initial_last_height(&self) -> Option<usize>;

    fn get_first_unsafe_height(&self) -> Option<usize>;

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
        + savefile::ReprC,
{
    #[inline(always)]
    fn get_initial_first_unsafe_height(&self) -> Option<usize> {
        // if self.initial_first_unsafe_height.is_none() {
        //     println!("{} NONE", &self.path);
        // } else {
        //     println!("{} some", &self.path);
        // }

        self.initial_first_unsafe_height
    }

    #[inline(always)]
    fn get_initial_last_height(&self) -> Option<usize> {
        self.initial_last_height
    }

    fn get_first_unsafe_height(&self) -> Option<usize> {
        last_height_to_first_unsafe_height(self.get_last_height())
    }

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync) {
        self
    }
}

impl<T> AnyExportableMap for HeightMap<T>
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
    fn export_then_clean(&self) -> color_eyre::Result<()> {
        self.export()?;

        self.clean_tmp_data();

        Ok(())
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
    fn prepare_tmp_data(&self) {
        if !self.modified.lock().to_owned() {
            return;
        }

        if self.storage == Storage::Disk {
            if self.inner.lock().is_some() {
                dbg!(&self.path);
                panic!("Probably forgot to drop inner after an export");
            }

            self.import_to_inner();

            self.batch.lock().drain(..).for_each(|(height, value)| {
                self.insert_to_inner(height, value);
            });
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

        if let Some(vec) = self.inner.lock().as_mut() {
            vec.clear()
        }

        *self.modified.lock() = false;

        Ok(())
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
    pub fn transform<F>(&self, transform: F) -> Vec<T>
    where
        T: Copy + Default,
        F: Fn((usize, &T, &[T])) -> T,
    {
        Self::_transform(self.inner.lock().as_ref().unwrap(), transform)
    }

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

    #[allow(unused)]
    pub fn add(&self, other: &Self) -> Vec<T>
    where
        T: Add<Output = T> + Copy + Default,
    {
        Self::_add(
            self.inner.lock().as_ref().unwrap(),
            other.inner.lock().as_ref().unwrap(),
        )
    }

    pub fn _add(arr1: &[T], arr2: &[T]) -> Vec<T>
    where
        T: Add<Output = T> + Copy + Default,
    {
        if arr1.len() != arr2.len() {
            panic!("Can't add two arrays with a different length");
        }

        Self::_transform(arr1, |(index, value, _)| *value + *arr2.get(index).unwrap())
    }

    pub fn subtract(&self, other: &Self) -> Vec<T>
    where
        T: Sub<Output = T> + Copy + Default,
    {
        Self::_subtract(
            self.inner.lock().as_ref().unwrap(),
            other.inner.lock().as_ref().unwrap(),
        )
    }

    pub fn _subtract(arr1: &[T], arr2: &[T]) -> Vec<T>
    where
        T: Sub<Output = T> + Copy + Default,
    {
        if arr1.len() != arr2.len() {
            panic!("Can't subtract two arrays with a different length");
        }

        Self::_transform(arr1, |(index, value, _)| *value - *arr2.get(index).unwrap())
    }

    pub fn multiply(&self, other: &Self) -> Vec<T>
    where
        T: Mul<Output = T> + Copy + Default,
    {
        Self::_multiply(
            self.inner.lock().as_ref().unwrap(),
            other.inner.lock().as_ref().unwrap(),
        )
    }

    pub fn _multiply(arr1: &[T], arr2: &[T]) -> Vec<T>
    where
        T: Mul<Output = T> + Copy + Default,
    {
        if arr1.len() != arr2.len() {
            panic!("Can't multiply two arrays with a different length");
        }

        Self::_transform(arr1, |(index, value, _)| *value * *arr2.get(index).unwrap())
    }

    pub fn divide(&self, other: &Self) -> Vec<T>
    where
        T: Div<Output = T> + Copy + Default,
    {
        Self::_divide(
            self.inner.lock().as_ref().unwrap(),
            other.inner.lock().as_ref().unwrap(),
        )
    }

    pub fn _divide(arr1: &[T], arr2: &[T]) -> Vec<T>
    where
        T: Div<Output = T> + Copy + Default,
    {
        if arr1.len() != arr2.len() {
            panic!("Can't divide two arrays with a different length");
        }

        Self::_transform(arr1, |(index, value, _)| *value / *arr2.get(index).unwrap())
    }

    pub fn cumulate(&self) -> Vec<T>
    where
        T: Sum + Copy + Default + AddAssign,
    {
        Self::_cumulate(self.inner.lock().as_ref().unwrap())
    }

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

    pub fn last_x_sum(&self, x: usize) -> Vec<T>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign,
    {
        Self::_last_x_sum(self.inner.lock().as_ref().unwrap(), x)
    }

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

    #[allow(unused)]
    pub fn moving_average(&self, x: usize) -> Vec<f32>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign + ToF32,
    {
        Self::_moving_average(self.inner.lock().as_ref().unwrap(), x)
    }

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

    pub fn net_change(&self, offset: usize) -> Vec<T>
    where
        T: Copy + Default + Sub<Output = T>,
    {
        Self::_net_change(self.inner.lock().as_ref().unwrap(), offset)
    }

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

    pub fn median(&self, size: usize) -> Vec<Option<T>>
    where
        T: FloatCore,
    {
        Self::_median(self.inner.lock().as_ref().unwrap(), size)
    }

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
