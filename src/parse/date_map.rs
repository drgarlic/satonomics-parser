use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use bincode::{Decode, Encode};
use chrono::{Days, NaiveDate};
use ordered_float::{FloatCore, OrderedFloat};
use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    io::{format_path, Serialization},
    utils::ToF32,
};

use super::{AnyExportableMap, AnyMap, Storage, WNaiveDate};

const NUMBER_OF_UNSAFE_DATES: usize = 2;

pub enum HeightToDateConverter<'a> {
    Last(&'a BTreeMap<WNaiveDate, usize>),
    Sum {
        date_to_first_height: &'a BTreeMap<WNaiveDate, usize>,
        date_to_last_height: &'a BTreeMap<WNaiveDate, usize>,
    },
    Manual,
}

pub struct DateMap<T> {
    storage: Storage,
    batch: Mutex<Vec<(NaiveDate, T)>>,
    path: String,
    initial_last_date: Option<NaiveDate>,
    initial_first_unsafe_date: Option<NaiveDate>,
    pub inner: Mutex<Option<BTreeMap<WNaiveDate, T>>>,
    modified: Mutex<bool>,
    serialization: Serialization,
}

impl<T> DateMap<T>
where
    T: Clone + Default + Encode + Decode + Debug + Serialize + DeserializeOwned + Sum,
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
            storage,
            batch: Mutex::new(vec![]),
            initial_last_date: None,
            initial_first_unsafe_date: None,
            path: serialization.append_extension(&format!("{path}/date")),
            inner: Mutex::new(None),
            modified: Mutex::new(false),
            serialization,
        };

        if storage == Storage::Memory {
            s.inner.lock().replace(s.import());
        }

        s.initial_last_date = s.get_last_date();
        s.initial_first_unsafe_date = last_date_to_first_unsafe_date(s.initial_last_date);

        s
    }

    pub fn insert(&self, date: NaiveDate, value: T) {
        if !self.is_date_safe(date) {
            *self.modified.lock() = true;

            if let Some(map) = self.inner.lock().as_mut() {
                map.insert(WNaiveDate::wrap(date), value);
            } else {
                self.batch.lock().push((date, value));
            }
        }
    }

    // pub fn insert_default(&self, date: NaiveDate) {
    //     self.insert(date, T::default())
    // }

    // pub fn compute_then_export_then_clean(
    //     &mut self,
    //     map: &[T],
    //     method: &HeightToDateConverter,
    // ) -> color_eyre::Result<()> {
    //     self.compute_from_height_map(map, method);
    //     self.export_then_clean()
    // }

    fn insert_to_inner(&self, date: NaiveDate, value: T) {
        self.inner
            .lock()
            .as_mut()
            .unwrap()
            .insert(WNaiveDate::wrap(date), value);
    }

    #[inline(always)]
    pub fn is_date_safe(&self, date: NaiveDate) -> bool {
        self.initial_first_unsafe_date
            .map_or(false, |initial_first_unsafe_date| {
                initial_first_unsafe_date > date
            })
    }

    // pub fn set_then_export_then_clean(
    //     &mut self,
    //     map: BTreeMap<WNaiveDate, T>,
    // ) -> color_eyre::Result<()> {
    //     self.set_inner(map);
    //     self.export_then_clean()
    // }

    pub fn set_inner(&self, map: BTreeMap<WNaiveDate, T>) {
        *self.modified.lock() = true;

        self.inner.lock().replace(map);
    }

    fn import_to_inner(&self) {
        self.set_inner(self.import());
    }

    fn import(&self) -> BTreeMap<WNaiveDate, T> {
        self.serialization.import(&self.path).unwrap_or_default()
    }

    fn get_first_unsafe_date(&self) -> Option<NaiveDate> {
        last_date_to_first_unsafe_date(self.get_last_date())
    }

    fn get_last_date(&self) -> Option<NaiveDate> {
        if let Some(inner) = self.inner.lock().as_ref() {
            inner.keys().map(|date| **date).max()
        } else {
            self.import().keys().map(|date| **date).max()
        }
    }

    pub fn compute_from_height_map(&self, map: &[T], converter: &HeightToDateConverter) {
        self.set_inner({
            match converter {
                HeightToDateConverter::Last(date_to_last_height) => date_to_last_height
                    .iter()
                    .map(|(date, height)| {
                        let v = map.get(*height).unwrap().clone();

                        (*date, v)
                    })
                    .collect(),
                HeightToDateConverter::Sum {
                    date_to_first_height,
                    date_to_last_height,
                } => date_to_first_height
                    .iter()
                    .map(|(date, height)| {
                        let v = map[*height..date_to_last_height.get(date).unwrap() + 1]
                            .iter()
                            .cloned()
                            .sum::<T>();

                        (*date, v)
                    })
                    .collect(),
                _ => todo!(),
            }
        });
    }

    fn export(&self) -> color_eyre::Result<()> {
        if !self.modified.lock().to_owned() {
            return Ok(());
        }

        *self.modified.lock() = false;

        if let Some(inner) = self.inner.lock().as_ref() {
            self.serialization.export(&self.path, inner)
        } else {
            if self.batch.lock().is_empty() {
                return Ok(());
            }

            let mut map = self.import();

            self.batch.lock().drain(..).for_each(|(date, value)| {
                map.insert(WNaiveDate::wrap(date), value);
            });

            self.serialization.export(&self.path, &map)
        }
    }

    fn clean_tmp_data(&self) {
        if self.storage == Storage::Disk {
            self.inner.lock().take();
        }
    }
}

pub trait AnyDateMap: AnyMap {
    fn get_initial_first_unsafe_date(&self) -> Option<NaiveDate>;

    fn get_initial_last_date(&self) -> Option<NaiveDate>;

    fn get_last_date(&self) -> Option<NaiveDate>;

    fn get_first_unsafe_date(&self) -> Option<NaiveDate>;

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync);
}

impl<T> AnyDateMap for DateMap<T>
where
    T: Clone + Default + Encode + Decode + Debug + Serialize + DeserializeOwned + Sum + Sync + Send,
{
    #[inline(always)]
    fn get_last_date(&self) -> Option<NaiveDate> {
        self.get_last_date()
    }

    #[inline(always)]
    fn get_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.get_first_unsafe_date()
    }

    #[inline(always)]
    fn get_initial_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.initial_first_unsafe_date
    }

    #[inline(always)]
    fn get_initial_last_date(&self) -> Option<NaiveDate> {
        self.initial_last_date
    }

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync) {
        self
    }
}

impl<T> AnyExportableMap for DateMap<T>
where
    T: Clone + Default + Encode + Decode + Debug + Serialize + DeserializeOwned + Sum,
{
    fn export_then_clean(&self) -> color_eyre::Result<()> {
        self.export()?;

        self.clean_tmp_data();

        Ok(())
    }
}

impl<T> AnyMap for DateMap<T>
where
    T: Clone + Default + Encode + Decode + Debug + Serialize + DeserializeOwned + Sum,
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

            self.batch.lock().drain(..).for_each(|(date, value)| {
                self.insert_to_inner(date, value);
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
        self.initial_last_date = None;
        self.initial_first_unsafe_date = None;

        if let Some(vec) = self.inner.lock().as_mut() {
            vec.clear()
        }

        *self.modified.lock() = false;

        Ok(())
    }
}

fn last_date_to_first_unsafe_date(last_date: Option<NaiveDate>) -> Option<NaiveDate> {
    last_date.and_then(|last_date| {
        let offset = NUMBER_OF_UNSAFE_DATES - 1;

        last_date.checked_sub_days(Days::new(offset as u64))
    })
}

impl<T> DateMap<T>
where
    T: Clone + Default + Debug + Decode + Encode + Serialize + DeserializeOwned + Sum,
{
    #[allow(unused)]
    pub fn transform<F>(&self, transform: F) -> BTreeMap<WNaiveDate, T>
    where
        T: Copy + Default,
        F: Fn((&WNaiveDate, &T, &BTreeMap<WNaiveDate, T>, usize)) -> T,
    {
        Self::_transform(self.inner.lock().as_ref().unwrap(), transform)
    }

    pub fn _transform<F>(map: &BTreeMap<WNaiveDate, T>, transform: F) -> BTreeMap<WNaiveDate, T>
    where
        T: Copy + Default,
        F: Fn((&WNaiveDate, &T, &BTreeMap<WNaiveDate, T>, usize)) -> T,
    {
        map.iter()
            .enumerate()
            .map(|(index, (date, value))| (date.to_owned(), transform((date, value, &map, index))))
            .collect()
    }

    #[allow(unused)]
    pub fn add(&self, other: &Self) -> BTreeMap<WNaiveDate, T>
    where
        T: Add<Output = T> + Copy + Default,
    {
        Self::_add(
            self.inner.lock().as_ref().unwrap(),
            other.inner.lock().as_ref().unwrap(),
        )
    }

    pub fn _add(
        map1: &BTreeMap<WNaiveDate, T>,
        map2: &BTreeMap<WNaiveDate, T>,
    ) -> BTreeMap<WNaiveDate, T>
    where
        T: Add<Output = T> + Copy + Default,
    {
        if map1.len() != map2.len() {
            panic!("Can't add two arrays with a different length");
        }

        Self::_transform(map1, |(date, value, ..)| *value + *map2.get(date).unwrap())
    }

    #[allow(unused)]
    pub fn subtract(&self, other: &Self) -> BTreeMap<WNaiveDate, T>
    where
        T: Sub<Output = T> + Copy + Default,
    {
        Self::_subtract(
            self.inner.lock().as_ref().unwrap(),
            other.inner.lock().as_ref().unwrap(),
        )
    }

    pub fn _subtract(
        map1: &BTreeMap<WNaiveDate, T>,
        map2: &BTreeMap<WNaiveDate, T>,
    ) -> BTreeMap<WNaiveDate, T>
    where
        T: Sub<Output = T> + Copy + Default,
    {
        if map1.len() != map2.len() {
            panic!("Can't subtract two arrays with a different length");
        }

        Self::_transform(map1, |(date, value, ..)| *value - *map2.get(date).unwrap())
    }

    #[allow(unused)]
    pub fn multiply(&self, other: &Self) -> BTreeMap<WNaiveDate, T>
    where
        T: Mul<Output = T> + Copy + Default,
    {
        Self::_multiply(
            self.inner.lock().as_ref().unwrap(),
            other.inner.lock().as_ref().unwrap(),
        )
    }

    pub fn _multiply(
        map1: &BTreeMap<WNaiveDate, T>,
        map2: &BTreeMap<WNaiveDate, T>,
    ) -> BTreeMap<WNaiveDate, T>
    where
        T: Mul<Output = T> + Copy + Default,
    {
        if map1.len() != map2.len() {
            panic!("Can't multiply two arrays with a different length");
        }

        Self::_transform(map1, |(date, value, ..)| *value * *map2.get(date).unwrap())
    }

    #[allow(unused)]
    pub fn divide(&self, other: &Self) -> BTreeMap<WNaiveDate, T>
    where
        T: Div<Output = T> + Copy + Default,
    {
        Self::_divide(
            self.inner.lock().as_ref().unwrap(),
            other.inner.lock().as_ref().unwrap(),
        )
    }

    pub fn _divide(
        map1: &BTreeMap<WNaiveDate, T>,
        map2: &BTreeMap<WNaiveDate, T>,
    ) -> BTreeMap<WNaiveDate, T>
    where
        T: Div<Output = T> + Copy + Default,
    {
        if map1.len() != map2.len() {
            panic!("Can't divide two arrays with a different length");
        }

        Self::_transform(map1, |(date, value, ..)| *value / *map2.get(date).unwrap())
    }

    #[allow(unused)]
    pub fn cumulate(&self) -> BTreeMap<WNaiveDate, T>
    where
        T: Sum + Copy + Default + AddAssign,
    {
        Self::_cumulate(self.inner.lock().as_ref().unwrap())
    }

    pub fn _cumulate(map: &BTreeMap<WNaiveDate, T>) -> BTreeMap<WNaiveDate, T>
    where
        T: Sum + Copy + Default + AddAssign,
    {
        let mut sum = T::default();

        map.iter()
            .map(|(date, value)| {
                sum += *value;
                (date.to_owned(), sum)
            })
            .collect()
    }

    #[allow(unused)]
    pub fn last_x_sum(&self, x: usize) -> BTreeMap<WNaiveDate, T>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign,
    {
        Self::_last_x_sum(self.inner.lock().as_ref().unwrap(), x)
    }

    pub fn _last_x_sum(map: &BTreeMap<WNaiveDate, T>, x: usize) -> BTreeMap<WNaiveDate, T>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign,
    {
        let mut sum = T::default();

        map.iter()
            .enumerate()
            .map(|(index, (date, value))| {
                sum += *value;

                if index >= x - 1 {
                    let previous_index = index + 1 - x;

                    sum -= *map.values().nth(previous_index).unwrap()
                }

                (date.to_owned(), sum)
            })
            .collect()
    }

    #[allow(unused)]
    pub fn simple_moving_average(&self, x: usize) -> BTreeMap<WNaiveDate, f32>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign + ToF32,
    {
        Self::_simple_moving_average(self.inner.lock().as_ref().unwrap(), x)
    }

    pub fn _simple_moving_average(
        map: &BTreeMap<WNaiveDate, T>,
        x: usize,
    ) -> BTreeMap<WNaiveDate, f32>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign + ToF32,
    {
        let mut sum = T::default();

        map.iter()
            .enumerate()
            .map(|(index, (date, value))| {
                sum += *value;

                if index >= x - 1 {
                    sum -= *map.values().nth(index + 1 - x).unwrap()
                }

                (date.to_owned(), sum.to_f32() / x as f32)
            })
            .collect()
    }

    #[allow(unused)]
    pub fn net_change(&self, offset: usize) -> BTreeMap<WNaiveDate, T>
    where
        T: Copy + Default + Sub<Output = T>,
    {
        Self::_net_change(self.inner.lock().as_ref().unwrap(), offset)
    }

    pub fn _net_change(map: &BTreeMap<WNaiveDate, T>, offset: usize) -> BTreeMap<WNaiveDate, T>
    where
        T: Copy + Default + Sub<Output = T>,
    {
        Self::_transform(map, |(_, value, map, index)| {
            let previous = {
                if let Some(previous_index) = index.checked_sub(offset) {
                    *map.values().nth(previous_index).unwrap()
                } else {
                    T::default()
                }
            };

            *value - previous
        })
    }

    #[allow(unused)]
    pub fn median(&self, size: usize) -> BTreeMap<WNaiveDate, Option<T>>
    where
        T: FloatCore,
    {
        Self::_median(self.inner.lock().as_ref().unwrap(), size)
    }

    pub fn _median(map: &BTreeMap<WNaiveDate, T>, size: usize) -> BTreeMap<WNaiveDate, Option<T>>
    where
        T: FloatCore,
    {
        let even = size % 2 == 0;
        let median_index = size / 2;

        if size < 3 {
            panic!("Computing a median for a size lower than 3 is useless");
        }

        map.iter()
            .enumerate()
            .map(|(index, (date, _))| {
                let value = {
                    if index >= size - 1 {
                        let starting_index = index - (size - 1);
                        let len = index + 1 - starting_index;

                        let mut chunks = map.values().skip(starting_index);

                        let mut vec = Vec::with_capacity(len);

                        for i in 0..len {
                            vec[i] = OrderedFloat(*chunks.next().unwrap());
                        }

                        vec.sort_unstable();

                        if even {
                            Some(
                                (**vec.get(median_index).unwrap()
                                    + **vec.get(median_index - 1).unwrap())
                                    / T::from(2.0).unwrap(),
                            )
                        } else {
                            Some(**vec.get(median_index).unwrap())
                        }
                    } else {
                        None
                    }
                };

                (date.to_owned(), value)
            })
            .collect()
    }
}
