use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs,
    iter::Sum,
    mem,
    ops::{Add, RangeInclusive, Sub},
    path::{Path, PathBuf},
};

use itertools::Itertools;
use ordered_float::{FloatCore, OrderedFloat};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    bitcoin::{BLOCKS_PER_HAVLING_EPOCH, NUMBER_OF_UNSAFE_BLOCKS},
    io::{format_path, Serialization},
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

    imported: BTreeMap<usize, BTreeMap<usize, T>>,
    to_insert: BTreeMap<usize, BTreeMap<usize, T>>,
}

impl<T> HeightMap<T>
where
    T: Clone
        + Copy
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

            to_insert: BTreeMap::default(),
            imported: BTreeMap::default(),
        };

        s.import_last();

        s.initial_last_height = s
            .imported
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

    pub fn insert(&mut self, height: usize, value: T) -> T {
        if !self.is_height_safe(height) {
            self.to_insert
                .entry(Self::height_to_chunk_start(height))
                .or_default()
                .insert(height, value);
        }

        value
    }

    pub fn insert_default(&mut self, height: usize) -> T {
        self.insert(height, T::default())
    }

    pub fn get(&self, height: &usize) -> Option<T> {
        let chunk_start = Self::height_to_chunk_start(*height);

        self.to_insert
            .get(&chunk_start)
            .and_then(|tree| tree.get(height).cloned())
            .or_else(|| {
                // DO that before
                // self.import_chunk_if_needed(*height);

                self.imported
                    .get(&chunk_start)
                    .and_then(|tree| tree.get(height))
                    .cloned()
            })
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

    pub fn import_if_needed(&mut self, height: usize) {
        let chunk_start = Self::height_to_chunk_start(height);

        if let Some(path) = self.read_dir().get(&chunk_start) {
            if self.imported.get(&chunk_start).is_none() {
                if let Ok(map) = self._import(path) {
                    self.imported.insert(chunk_start, map);
                }
            }
        }
    }

    fn import_last(&mut self) {
        if let Some((chunk_start, path)) = self.read_dir().into_iter().last() {
            if let Ok(map) = self._import(&path) {
                self.imported.insert(chunk_start, map);
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
        + Copy
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
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

        self.imported.clear();
        self.to_insert.clear();

        Ok(())
    }

    fn pre_export(&mut self) {
        // Don't take to_insert, only take map (which should be vecs really)
        // use to_insert keys to know what to export
        // Then drop to_insert in post_export
        //
        if true {
            panic!("do upper stuff");
        }

        if let Some(first_map_to_insert) = self.to_insert.iter().next() {
            let first_height_to_insert = first_map_to_insert.1.iter().next().unwrap().0;

            if first_height_to_insert % CHUNK_SIZE != 0 {
                self.import_if_needed(*first_height_to_insert);
            }
        }

        let imported = &mut self.imported;

        self.to_insert
            .iter_mut()
            .enumerate()
            .for_each(|(_, (chunk_start, map))| {
                let to_export = imported.entry(chunk_start.to_owned()).or_default();

                to_export.extend(mem::take(map));
            });
    }

    fn export(&self) -> color_eyre::Result<()> {
        let len = self.imported.len();

        self.to_insert.iter().enumerate().try_for_each(
            |(index, (chunk_start, _))| -> color_eyre::Result<()> {
                let chunk_name = Self::height_to_chunk_name(*chunk_start);

                let path = self
                    .serialization
                    .append_extension(&format!("{}/{}", self.path_all, chunk_name));

                let vec = self
                    .imported
                    .get(chunk_start)
                    .unwrap()
                    .values()
                    .cloned()
                    .collect_vec();

                self.serialization.export(&path, &vec)?;

                if index == len - 1 {
                    if let Some(path_last) = self.path_last.as_ref() {
                        self.serialization.export(path_last, vec.last().unwrap())?;
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
            .filter(|(index, _)| index + 1 < len)
            .for_each(|(_, key)| {
                self.imported.remove(&key);
            });

        self.to_insert.clear();
    }
}

pub trait AnyHeightMap: AnyMap {
    fn get_initial_first_unsafe_height(&self) -> Option<usize>;

    fn get_initial_last_height(&self) -> Option<usize>;

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync);

    fn as_any_mut_map(&mut self) -> &mut dyn AnyMap;
}

impl<T> AnyHeightMap for HeightMap<T>
where
    T: Clone
        + Copy
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

    fn as_any_mut_map(&mut self) -> &mut dyn AnyMap {
        self
    }
}

impl<T> HeightMap<T>
where
    T: Clone
        + Copy
        + Default
        + Debug
        + Serialize
        + DeserializeOwned
        + savefile::Serialize
        + savefile::Deserialize
        + savefile::ReprC,
{
    pub fn sum_range(&self, range: &RangeInclusive<usize>) -> T
    where
        T: Sum,
    {
        range
            .to_owned()
            .flat_map(|height| self.get(&height))
            .sum::<T>()
    }

    pub fn insert_cumulative(&mut self, height: usize, source: &HeightMap<T>) -> T
    where
        T: Add<Output = T> + Sub<Output = T>,
    {
        let previous_cum = height
            .checked_sub(1)
            .map(|previous_sum_height| {
                self.get(&previous_sum_height).unwrap_or_else(|| {
                    dbg!(previous_sum_height);
                    panic!()
                })
            })
            .unwrap_or_default();

        let last_value = source.get(&height).unwrap();

        let cum_value = previous_cum + last_value;

        self.insert(height, cum_value);

        cum_value
    }

    pub fn insert_last_x_sum(&mut self, height: usize, source: &HeightMap<T>, x: usize) -> T
    where
        T: Add<Output = T> + Sub<Output = T>,
    {
        let to_subtract = (height + 1)
            .checked_sub(x)
            .map(|previous_height| source.get(&previous_height).unwrap())
            .unwrap_or_default();

        let previous_sum = height
            .checked_sub(1)
            .map(|previous_sum_height| self.get(&previous_sum_height).unwrap())
            .unwrap_or_default();

        let last_value = source.get(&height).unwrap();

        let sum = previous_sum - to_subtract + last_value;

        self.insert(height, sum);

        sum
    }

    #[allow(unused)]
    pub fn insert_simple_average(&mut self, height: usize, source: &HeightMap<T>, x: usize)
    where
        T: Into<f32> + From<f32>,
    {
        let to_subtract: f32 = (height + 1)
            .checked_sub(x)
            .map(|previous_height| source.get(&previous_height).unwrap())
            .unwrap_or_default()
            .into();

        let previous_average: f32 = height
            .checked_sub(1)
            .map(|previous_average_height| self.get(&previous_average_height).unwrap())
            .unwrap_or_default()
            .into();

        let last_value: f32 = source.get(&height).unwrap().into();

        let sum = previous_average * x as f32 - to_subtract + last_value;

        let average: T = (sum / x as f32).into();

        self.insert(height, average);
    }

    pub fn insert_net_change(&mut self, height: usize, source: &HeightMap<T>, offset: usize) -> T
    where
        T: Sub<Output = T>,
    {
        let previous_value = height
            .checked_sub(offset)
            .map(|height| source.get(&height).unwrap())
            .unwrap_or_default();

        let last_value = source.get(&height).unwrap();

        let net = last_value - previous_value;

        self.insert(height, net);

        net
    }

    pub fn insert_median(&mut self, height: usize, source: &HeightMap<T>, size: usize) -> T
    where
        T: FloatCore,
    {
        if size < 3 {
            panic!("Computing a median for a size lower than 3 is useless");
        }

        let median = {
            if let Some(start) = height.checked_sub(size - 1) {
                let even = size % 2 == 0;
                let median_index = size / 2;

                let mut vec = (start..=height)
                    .flat_map(|height| source.get(&height))
                    .map(|f| OrderedFloat(f))
                    .collect_vec();

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

        self.insert(height, median);

        median
    }
}
