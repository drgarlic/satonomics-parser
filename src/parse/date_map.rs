use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs,
    iter::Sum,
    mem,
    ops::{Add, AddAssign, DerefMut, Div, Mul, Sub, SubAssign},
    path::{Path, PathBuf},
};

use chrono::{Datelike, Days, NaiveDate};
use itertools::Itertools;
use ordered_float::{FloatCore, OrderedFloat};
use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    io::{format_path, Serialization},
    utils::ToF32,
};

use super::{AnyMap, WNaiveDate};

const NUMBER_OF_UNSAFE_DATES: usize = 2;

pub enum HeightToDateConverter<'a> {
    Last(&'a DateMap<usize>),
    Sum {
        first_height: &'a DateMap<usize>,
        last_height: &'a DateMap<usize>,
    },
}

pub struct DateMap<T> {
    path_all: String,
    path_last: Option<String>,

    serialization: Serialization,

    initial_last_date: Option<NaiveDate>,
    initial_first_unsafe_date: Option<NaiveDate>,

    imported: Mutex<BTreeMap<usize, BTreeMap<WNaiveDate, T>>>,
    to_insert: Mutex<BTreeMap<usize, BTreeMap<WNaiveDate, T>>>,
}

impl<T> DateMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + savefile::Serialize
        + savefile::Deserialize,
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
            path_all,
            path_last,

            serialization,

            initial_last_date: None,
            initial_first_unsafe_date: None,

            to_insert: Mutex::new(BTreeMap::default()),
            imported: Mutex::new(BTreeMap::default()),
        };

        s.import_last();

        s.initial_last_date = s
            .imported
            .lock()
            .values()
            .last()
            .and_then(|d| d.keys().map(|date| **date).max());

        s.initial_first_unsafe_date = s.initial_last_date.and_then(|last_date| {
            let offset = NUMBER_OF_UNSAFE_DATES - 1;
            last_date.checked_sub_days(Days::new(offset as u64))
        });

        s
    }

    pub fn insert(&self, date: NaiveDate, value: T) {
        if !self.is_date_safe(date) {
            self.to_insert
                .lock()
                .entry(date.year() as usize)
                .or_default()
                .insert(WNaiveDate::wrap(date), value);
        } else {
            panic!("Shouldn't have called insert")
        }
    }

    #[allow(unused)]
    pub fn insert_default(&self, date: NaiveDate) {
        self.insert(date, T::default())
    }

    pub fn get(&self, date: &WNaiveDate) -> Option<T> {
        let year = date.year() as usize;

        self.to_insert
            .lock()
            .get(&year)
            .and_then(|tree| tree.get(date).cloned())
            .or_else(|| {
                self.import_year_if_needed(year);

                self.imported
                    .lock()
                    .get(&year)
                    .and_then(|tree| tree.get(date))
                    .cloned()
            })
    }

    // pub fn get_range(&self, first_date: usize, last_date: usize) -> Vec<Option<T>> {
    //     let to_insert = self.to_insert.lock();
    //     let imported = &mut None;

    //     (first_height..=last_height)
    //         .map(|height| {
    //             let chunk_name = Self::height_to_chunk_name(height);

    //             to_insert
    //                 .get(&chunk_name)
    //                 .and_then(|tree| tree.get(&height).cloned())
    //                 .or_else(|| {
    //                     self.import_chunk_if_needed(&chunk_name);

    //                     if imported.is_none() {
    //                         imported.as_mut().replace(&mut self.imported.lock());
    //                     }

    //                     imported
    //                         .as_mut()
    //                         .unwrap()
    //                         .get(&chunk_name)
    //                         .and_then(|tree| tree.get(&height))
    //                         .cloned()
    //                 })
    //         })
    //         .collect_vec()
    // }

    #[inline(always)]
    pub fn is_date_safe(&self, date: NaiveDate) -> bool {
        self.initial_first_unsafe_date
            .map_or(false, |initial_first_unsafe_date| {
                initial_first_unsafe_date > date
            })
    }

    fn read_dir(&self) -> BTreeMap<usize, PathBuf> {
        fs::read_dir(&self.path_all)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let extension = path.extension().unwrap().to_str().unwrap();

                path.is_file()
                    && file_stem.len() == 4
                    && file_stem.starts_with("20")
                    && extension == self.serialization.to_str()
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

    // fn import_all(&self) -> BTreeMap<WNaiveDate, T> {
    //     self.read_dir()
    //         .into_values()
    //         .map(|path| {
    //             self.serialization
    //                 .import::<BTreeMap<WNaiveDate, T>>(path.to_str().unwrap())
    //                 .unwrap()
    //         })
    //         .reduce(|mut a, mut b| {
    //             a.extend(b);
    //             a
    //         })
    //         .unwrap_or_default()
    // }

    fn import_year_if_needed(&self, year: usize) {
        if let Some(path) = self.read_dir().get(&year) {
            let mut imported = self.imported.lock();

            if imported.get(&year).is_none() {
                if let Ok(map) = self._import(path) {
                    imported.insert(year, map);
                }
            }
        }
    }

    fn import_last(&self) {
        if let Some((year, path)) = self.read_dir().into_iter().last() {
            if let Ok(map) = self._import(&path) {
                self.imported.lock().insert(year, map);
            }
        }
    }

    fn _import(&self, path: &Path) -> color_eyre::Result<BTreeMap<WNaiveDate, T>> {
        self.serialization
            .import::<BTreeMap<WNaiveDate, T>>(path.to_str().unwrap())
    }

    // pub fn compute_from_height_map(&self, map: &[T], converter: &HeightToDateConverter) {
    //     self.set_inner({
    //         match converter {
    //             HeightToDateConverter::Last(last_height) => {
    //                 let last_height = last_height.imported.lock();

    //                 last_height
    //                     .as_ref()
    //                     .unwrap()
    //                     .iter()
    //                     .map(|(date, height)| {
    //                         let v = map.get(*height).unwrap().clone();

    //                         (*date, v)
    //                     })
    //                     .collect()
    //             }
    //             HeightToDateConverter::Sum {
    //                 first_height,
    //                 last_height,
    //             } => {
    //                 let last_height = last_height.imported.lock();

    //                 let first_height = first_height.imported.lock();

    //                 first_height
    //                     .as_ref()
    //                     .unwrap()
    //                     .iter()
    //                     .map(|(date, height)| {
    //                         let v = map
    //                             [*height..last_height.as_ref().unwrap().get(date).unwrap() + 1]
    //                             .iter()
    //                             .cloned()
    //                             .sum::<T>();

    //                         (*date, v)
    //                     })
    //                     .collect()
    //             }
    //         }
    //     });
    // }
}

impl<T> AnyMap for DateMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + savefile::Serialize
        + savefile::Deserialize,
{
    //     fn import_tmp_data(&self) {
    //         // println!("import tmp {}", &self.path);

    //         if !self.modified.lock().to_owned() {
    //             return;
    //         }

    //         if self.storage == Storage::Disk {
    //             if self.imported.lock().is_some() {
    //                 dbg!(&self.path);
    //                 panic!("Probably forgot to drop inner after an export");
    //             }

    //             self.import_to_inner();

    //             self.to_insert.lock().drain(..).for_each(|(date, value)| {
    //                 self.insert_to_inner(date, value);
    //             });
    //         }
    //     }

    fn path(&self) -> &str {
        &self.path_all
    }

    fn t_name(&self) -> &str {
        std::any::type_name::<T>()
    }

    fn reset(&mut self) -> color_eyre::Result<()> {
        fs::remove_dir(&self.path_all)?;

        self.initial_last_date = None;
        self.initial_first_unsafe_date = None;

        self.imported.lock().clear();
        self.to_insert.lock().clear();

        Ok(())
    }

    fn export(&self) -> color_eyre::Result<()> {
        let to_insert = mem::take(self.to_insert.lock().deref_mut());

        match to_insert.iter().next() {
            Some(first_map_to_insert) => {
                let first_date_to_insert = **first_map_to_insert.1.iter().next().unwrap().0;

                let day = first_date_to_insert.day();
                let month = first_date_to_insert.month();
                let year = first_date_to_insert.year() as usize;

                let is_first_of_year = month == 1 && (day == 1 || (day == 3 && year == 2009));

                if !is_first_of_year {
                    self.import_year_if_needed(year)
                }
            }
            None => return Ok(()),
        }

        let mut imported = self.imported.lock();

        let len = imported.len();

        to_insert.into_iter().enumerate().try_for_each(
            |(index, (year, map))| -> color_eyre::Result<()> {
                let to_export = imported.entry(year).or_default();

                to_export.extend(map);

                let path = self
                    .serialization
                    .append_extension(&format!("{}/{}", self.path_all, year));

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
        let mut imported = self.imported.lock();

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

pub trait AnyDateMap: AnyMap {
    fn get_initial_first_unsafe_date(&self) -> Option<NaiveDate>;

    fn get_initial_last_date(&self) -> Option<NaiveDate>;

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync);
}

impl<T> AnyDateMap for DateMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + Sync
        + Send
        + savefile::Serialize
        + savefile::Deserialize,
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
}

impl<T> DateMap<T>
where
    T: Clone
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + Sum
        + savefile::Serialize
        + savefile::Deserialize,
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

    #[allow(unused)]
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

    // #[allow(unused)]
    // pub fn net_change(&self, offset: usize) -> BTreeMap<WNaiveDate, T>
    // where
    //     T: Copy + Default + Sub<Output = T>,
    // {
    //     Self::_net_change(self.imported.lock().as_ref().unwrap(), offset)
    // }

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
