use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs,
    iter::Sum,
    mem,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
    path::{Path, PathBuf},
    str::FromStr,
};

use chrono::{Datelike, Days, NaiveDate};
use itertools::Itertools;
use ordered_float::{FloatCore, OrderedFloat};
use savefile_derive::Savefile;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    io::{format_path, Serialization},
    utils::ToF32,
};

use super::{AnyMap, WNaiveDate};

const NUMBER_OF_UNSAFE_DATES: usize = 2;

#[derive(Debug, Savefile, Serialize, Deserialize)]
pub struct SerializedDateMap<T> {
    version: u32,
    map: BTreeMap<WNaiveDate, T>,
}

pub struct DateMap<T> {
    version: u32,

    path_all: String,
    path_last: Option<String>,

    chunks_in_memory: usize,

    serialization: Serialization,

    initial_last_date: Option<NaiveDate>,
    initial_first_unsafe_date: Option<NaiveDate>,

    imported: BTreeMap<usize, SerializedDateMap<T>>,
    to_insert: BTreeMap<usize, BTreeMap<WNaiveDate, T>>,
}

impl<T> DateMap<T>
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
    #[allow(unused)]
    pub fn new_bin(version: u32, path: &str) -> Self {
        Self::new(version, path, Serialization::Binary, 1, true)
    }

    #[allow(unused)]
    pub fn _new_bin(version: u32, path: &str, chunks_in_memory: usize, export_last: bool) -> Self {
        Self::new(
            version,
            path,
            Serialization::Binary,
            chunks_in_memory,
            export_last,
        )
    }

    #[allow(unused)]
    pub fn new_json(version: u32, path: &str) -> Self {
        Self::new(version, path, Serialization::Json, 1, true)
    }

    #[allow(unused)]
    pub fn _new_json(version: u32, path: &str, chunks_in_memory: usize, export_last: bool) -> Self {
        Self::new(
            version,
            path,
            Serialization::Json,
            chunks_in_memory,
            export_last,
        )
    }

    fn new(
        version: u32,
        path: &str,
        serialization: Serialization,
        chunks_in_memory: usize,
        export_last: bool,
    ) -> Self {
        if chunks_in_memory < 1 {
            panic!("Should always have at least the latest chunk in memory");
        }

        let path = format_path(path);

        let path_all = format!("{path}/date");

        fs::create_dir_all(&path_all).unwrap();

        let path_last = {
            if export_last {
                Some(serialization.append_extension(&format!("{path}/last")))
            } else {
                None
            }
        };

        let mut s = Self {
            version,

            path_all,
            path_last,

            chunks_in_memory,

            serialization,

            initial_last_date: None,
            initial_first_unsafe_date: None,

            to_insert: BTreeMap::default(),
            imported: BTreeMap::default(),
        };

        s.read_dir()
            .into_iter()
            .rev()
            .take(chunks_in_memory)
            .for_each(|(chunk_start, path)| {
                if let Ok(serialized) = s.import(&path) {
                    if serialized.version == s.version {
                        s.imported.insert(chunk_start, serialized);
                    }
                }
            });

        s.initial_last_date = s
            .imported
            .values()
            .last()
            .and_then(|serialized| serialized.map.keys().map(|date| **date).max());

        s.initial_first_unsafe_date = s.initial_last_date.and_then(|last_date| {
            let offset = NUMBER_OF_UNSAFE_DATES - 1;
            last_date.checked_sub_days(Days::new(offset as u64))
        });

        s
    }

    pub fn insert(&mut self, date: NaiveDate, value: T) -> T {
        if !self.is_date_safe(date) {
            self.to_insert
                .entry(date.year() as usize)
                .or_default()
                .insert(WNaiveDate::wrap(date), value);
        }

        value
    }

    pub fn insert_default(&mut self, date: NaiveDate) -> T {
        self.insert(date, T::default())
    }

    pub fn get(&self, date: NaiveDate) -> Option<T> {
        self._get(&WNaiveDate::wrap(date))
    }

    pub fn _get(&self, date: &WNaiveDate) -> Option<T> {
        let year = date.year() as usize;

        self.to_insert
            .get(&year)
            .and_then(|tree| tree.get(date).cloned())
            .or_else(|| {
                self.imported
                    .get(&year)
                    .and_then(|serialized| serialized.map.get(date))
                    .cloned()
            })
    }

    #[inline(always)]
    pub fn is_date_safe(&self, date: NaiveDate) -> bool {
        self.initial_first_unsafe_date
            .map_or(false, |initial_first_unsafe_date| {
                initial_first_unsafe_date > date
            })
    }

    fn read_dir(&self) -> BTreeMap<usize, PathBuf> {
        Self::_read_dir(&self.path_all, &self.serialization)
    }

    pub fn _read_dir(path: &str, serialization: &Serialization) -> BTreeMap<usize, PathBuf> {
        fs::read_dir(path)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let extension = path.extension().unwrap().to_str().unwrap();

                path.is_file()
                    && file_stem.len() == 4
                    && file_stem.starts_with("20")
                    && extension == serialization.to_extension()
            })
            .map(|path| {
                let year = path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                (year, path)
            })
            .collect()
    }

    fn import(&self, path: &Path) -> color_eyre::Result<SerializedDateMap<T>> {
        self.serialization
            .import::<SerializedDateMap<T>>(path.to_str().unwrap())
    }
}

impl<T> AnyMap for DateMap<T>
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
    fn path(&self) -> &str {
        &self.path_all
    }

    fn path_last(&self) -> &Option<String> {
        &self.path_last
    }

    fn t_name(&self) -> &str {
        std::any::type_name::<T>()
    }

    fn reset(&mut self) -> color_eyre::Result<()> {
        fs::remove_dir(&self.path_all)?;

        self.initial_last_date = None;
        self.initial_first_unsafe_date = None;

        self.imported.clear();
        self.to_insert.clear();

        Ok(())
    }

    fn pre_export(&mut self) {
        self.to_insert
            .iter_mut()
            .enumerate()
            .for_each(|(_, (chunk_start, map))| {
                self.imported
                    .entry(chunk_start.to_owned())
                    .or_insert(SerializedDateMap {
                        version: self.version,
                        map: BTreeMap::default(),
                    })
                    .map
                    .extend(mem::take(map));
            });
    }

    fn export(&self) -> color_eyre::Result<()> {
        let len = self.imported.len();

        self.to_insert.iter().enumerate().try_for_each(
            |(index, (year, _))| -> color_eyre::Result<()> {
                let path = self
                    .serialization
                    .append_extension(&format!("{}/{}", self.path_all, year));

                let serialized = self.imported.get(year).unwrap();

                self.serialization.export(&path, serialized)?;

                if index == len - 1 {
                    if let Some(path_last) = self.path_last.as_ref() {
                        self.serialization
                            .export(path_last, serialized.map.values().last().unwrap())?;
                    }
                }

                Ok(())
            },
        )
    }

    fn post_export(&mut self) {
        let len = self.imported.len();

        let keys = self.imported.keys().cloned().collect_vec();

        keys.into_iter()
            .enumerate()
            .filter(|(index, _)| index + self.chunks_in_memory < len)
            .for_each(|(_, key)| {
                self.imported.remove(&key);
            });

        self.to_insert.clear();
    }
}

pub trait AnyDateMap: AnyMap {
    fn get_initial_first_unsafe_date(&self) -> Option<NaiveDate>;

    fn get_initial_last_date(&self) -> Option<NaiveDate>;

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync);

    fn as_any_mut_map(&mut self) -> &mut dyn AnyMap;
}

impl<T> AnyDateMap for DateMap<T>
where
    T: Clone
        + Copy
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + Sync
        + Send
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
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

    fn as_any_mut_map(&mut self) -> &mut dyn AnyMap {
        self
    }
}

impl<T> DateMap<T>
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
    // #[allow(unused)]
    // pub fn transform<F>(&self, transform: F) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Copy + Default,
    //     F: Fn((&WNaiveDate, &T, &BTreeMap<WNaiveDate, T>, usize)) -> T,
    // {
    //     Self::_transform(self.imported.lock().as_ref().unwrap(), transform)
    // }

    #[allow(unused)]
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

    // #[allow(unused)]
    // pub fn add(&self, other: &Self) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Add<Output = T> + Copy + Default,
    // {
    //     Self::_add(
    //         self.imported.lock().as_ref().unwrap(),
    //         other.imported.lock().as_ref().unwrap(),
    //     )
    // }

    pub fn _add(
        map1: &BTreeMap<WNaiveDate, T>,
        map2: &BTreeMap<WNaiveDate, T>,
    ) -> BTreeMap<WNaiveDate, T>
    where
        T: Add<Output = T> + Copy + Default,
    {
        Self::_transform(map1, |(date, value, ..)| {
            map2.get(date)
                .map(|value2| *value + *value2)
                .unwrap_or_default()
        })
    }

    // #[allow(unused)]
    // pub fn subtract(&self, other: &Self) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Sub<Output = T> + Copy + Default,
    // {
    //     Self::_subtract(
    //         self.imported.lock().as_ref().unwrap(),
    //         other.imported.lock().as_ref().unwrap(),
    //     )
    // }

    #[allow(unused)]
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

        Self::_transform(map1, |(date, value, ..)| {
            map2.get(date)
                .map(|value2| *value - *value2)
                .unwrap_or_default()
        })
    }

    // #[allow(unused)]
    // pub fn multiply(&self, other: &Self) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Mul<Output = T> + Copy + Default,
    // {
    //     Self::_multiply(
    //         self.imported.lock().as_ref().unwrap(),
    //         other.imported.lock().as_ref().unwrap(),
    //     )
    // }

    #[allow(unused)]
    pub fn _multiply(
        map1: &BTreeMap<WNaiveDate, T>,
        map2: &BTreeMap<WNaiveDate, T>,
    ) -> BTreeMap<WNaiveDate, T>
    where
        T: Mul<Output = T> + Copy + Default,
    {
        Self::_transform(map1, |(date, value, ..)| {
            map2.get(date)
                .map(|value2| *value * *value2)
                .unwrap_or_default()
        })
    }

    // #[allow(unused)]
    // pub fn divide(&self, other: &Self) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Div<Output = T> + Copy + Default,
    // {
    //     Self::_divide(
    //         self.imported.lock().as_ref().unwrap(),
    //         other.imported.lock().as_ref().unwrap(),
    //     )
    // }

    #[allow(unused)]
    pub fn _divide(
        map1: &BTreeMap<WNaiveDate, T>,
        map2: &BTreeMap<WNaiveDate, T>,
    ) -> BTreeMap<WNaiveDate, T>
    where
        T: Div<Output = T> + Copy + Default,
    {
        Self::_transform(map1, |(date, value, ..)| {
            map2.get(date)
                .map(|value2| *value / *value2)
                .unwrap_or_default()
        })
    }

    // #[allow(unused)]
    // pub fn cumulate(&self) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Sum + Copy + Default + AddAssign,
    // {
    //     Self::_cumulate(self.imported.lock().as_ref().unwrap())
    // }

    #[allow(unused)]
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

    pub fn insert_cumulative(&mut self, date: NaiveDate, source: &DateMap<T>) -> T
    where
        T: Add<Output = T> + Sub<Output = T>,
    {
        let previous_cum = date
            .checked_sub_days(Days::new(1))
            .map(|previous_date| {
                self.get(previous_date).unwrap_or_else(|| {
                    if previous_date.year() == 2009 && previous_date.month() == 1 {
                        let day = previous_date.day();

                        if day == 8 {
                            self.get(NaiveDate::from_str("2009-01-03").unwrap())
                                .unwrap()
                        } else if day == 2 {
                            T::default()
                        } else {
                            panic!()
                        }
                    } else {
                        dbg!(previous_date, &self.path_all);
                        panic!()
                    }
                })
            })
            .unwrap_or_default();

        let last_value = source.get(date).unwrap();

        let cum_value = previous_cum + last_value;

        self.insert(date, cum_value);

        cum_value
    }

    #[allow(unused)]
    pub fn insert_last_x_sum(&mut self, date: NaiveDate, source: &DateMap<T>, x: usize) -> T
    where
        T: Add<Output = T> + Sub<Output = T>,
    {
        let to_subtract = date
            .checked_sub_days(Days::new(x as u64 - 1))
            .and_then(|previous_date| source.get(previous_date))
            .unwrap_or_default();

        let previous_sum = date
            .checked_sub_days(Days::new(1))
            .and_then(|previous_sum_date| self.get(previous_sum_date))
            .unwrap_or_default();

        let last_value = source.get(date).unwrap();

        let sum = previous_sum - to_subtract + last_value;

        self.insert(date, sum);

        sum
    }

    // #[allow(unused)]
    // pub fn last_x_sum(&self, x: usize) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Sum + Copy + Default + AddAssign + SubAssign,
    // {
    //     Self::_last_x_sum(self.imported.lock().as_ref().unwrap(), x)
    // }

    #[allow(unused)]
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

    // #[allow(unused)]
    // pub fn simple_moving_average(&self, x: usize) -> BTreeMap<WNaiveDate, f32>
    // where
    //     T: Sum + Copy + Default + AddAssign + SubAssign + ToF32,
    // {
    //     Self::_simple_moving_average(self.imported.lock().as_ref().unwrap(), x)
    // }
    //
    #[allow(unused)]
    pub fn insert_simple_average<K>(&mut self, date: NaiveDate, source: &DateMap<K>, x: usize)
    where
        T: Into<f32> + From<f32>,
        K: Clone
            + Copy
            + Default
            + Debug
            + Serialize
            + DeserializeOwned
            + Sum
            + savefile::Serialize
            + savefile::Deserialize
            + savefile::ReprC
            + ToF32,
    {
        let to_subtract = date
            .checked_sub_days(Days::new(x as u64 - 1))
            .and_then(|previous_date| source.get(previous_date))
            .unwrap_or_default()
            .to_f32();

        let previous_average: f32 = date
            .checked_sub_days(Days::new(1))
            .and_then(|previous_average_date| self.get(previous_average_date))
            .unwrap_or_default()
            .into();

        let last_value: f32 = source.get(date).unwrap().to_f32();

        let sum = previous_average * x as f32 - 0.0 + last_value;

        let average: T = (sum / x as f32).into();

        self.insert(date, average);
    }

    #[allow(unused)]
    pub fn _simple_moving_average(
        map: &BTreeMap<WNaiveDate, T>,
        x: usize,
    ) -> BTreeMap<WNaiveDate, f32>
    where
        T: Sum + Copy + Default + AddAssign + SubAssign + Into<f32>,
    {
        let mut sum = T::default();

        map.iter()
            .enumerate()
            .map(|(index, (date, value))| {
                sum += *value;

                if index >= x - 1 {
                    sum -= *map.values().nth(index + 1 - x).unwrap()
                }

                let float_sum: f32 = sum.into();

                (date.to_owned(), float_sum / x as f32)
            })
            .collect()
    }

    // #[allow(unused)]
    // pub fn net_change(&self, offset: usize) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Copy + Default + Sub<Output = T>,
    // {
    //     Self::_net_change(self.imported.lock().as_ref().unwrap(), offset)
    // }
    //
    //
    pub fn insert_net_change(&mut self, date: NaiveDate, source: &DateMap<T>, offset: usize) -> T
    where
        T: Sub<Output = T>,
    {
        let previous_value = date
            .checked_sub_days(Days::new(offset as u64))
            .and_then(|date| source.get(date))
            .unwrap_or_default();

        let last_value = source.get(date).unwrap_or_else(|| {
            dbg!(date);
            panic!();
        });

        let net = last_value - previous_value;

        self.insert(date, net);

        net
    }

    #[allow(unused)]
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

    // #[allow(unused)]
    // pub fn median(&self, size: usize) -> BTreeMap<WNaiveDate, Option<T>>
    // where
    //     T: FloatCore,
    // {
    //     Self::_median(self.imported.lock().as_ref().unwrap(), size)
    // }
    //
    pub fn insert_median(&mut self, date: NaiveDate, source: &DateMap<T>, size: usize) -> T
    where
        T: FloatCore,
    {
        if size < 3 {
            panic!("Computing a median for a size lower than 3 is useless");
        }

        let median = {
            if let Some(start) = date.checked_sub_days(Days::new(size as u64 - 1)) {
                let even = size % 2 == 0;
                let median_index = size / 2;

                let mut vec = start
                    .iter_days()
                    .take(size)
                    .flat_map(|date| source.get(date))
                    .map(|f| OrderedFloat(f))
                    .collect_vec();

                if vec.len() != size {
                    return T::default();
                }

                vec.sort_unstable();

                if even {
                    (vec.get(median_index).unwrap().0 + vec.get(median_index - 1).unwrap().0)
                        / T::from(2.0).unwrap()
                } else {
                    vec.get(median_index).unwrap().0
                }
            } else {
                T::default()
            }
        };

        self.insert(date, median);

        median
    }

    #[allow(unused)]
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
                        let mut vec = map
                            .values()
                            .rev()
                            .take(size)
                            .map(|v| OrderedFloat(*v))
                            .collect_vec();

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
