use std::{
    cmp::Ordering,
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
use savefile_derive::Savefile;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    bitcoin::{BLOCKS_PER_HAVLING_EPOCH, NUMBER_OF_UNSAFE_BLOCKS},
    io::{format_path, Serialization},
};

use super::AnyMap;

pub const HEIGHT_MAP_CHUNK_SIZE: usize = BLOCKS_PER_HAVLING_EPOCH / 16;

#[derive(Debug, Savefile, Serialize, Deserialize)]
pub struct SerializedHeightMap<T> {
    version: u32,
    map: Vec<T>,
}

pub struct HeightMap<T>
where
    T: Clone + Default + Debug + savefile::Serialize + savefile::Deserialize,
{
    version: u32,

    path_all: String,
    path_last: Option<String>,

    chunks_in_memory: usize,

    serialization: Serialization,

    initial_last_height: Option<usize>,
    initial_first_unsafe_height: Option<usize>,

    imported: BTreeMap<usize, SerializedHeightMap<T>>,
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
            version,

            path_all,
            path_last,

            chunks_in_memory,

            serialization,

            initial_first_unsafe_height: None,
            initial_last_height: None,

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

        s.initial_last_height = s
            .imported
            .iter()
            .last()
            .map(|(chunk_start, serialized)| chunk_start + serialized.map.len());

        s.initial_first_unsafe_height = s.initial_last_height.and_then(|last_height| {
            let offset = NUMBER_OF_UNSAFE_BLOCKS - 1;
            last_height.checked_sub(offset)
        });

        s
    }

    fn height_to_chunk_name(height: usize) -> String {
        let start = Self::height_to_chunk_start(height);
        let end = start + HEIGHT_MAP_CHUNK_SIZE;

        format!("{start}..{end}")
    }

    fn height_to_chunk_start(height: usize) -> usize {
        height / HEIGHT_MAP_CHUNK_SIZE * HEIGHT_MAP_CHUNK_SIZE
    }

    pub fn insert(&mut self, height: usize, value: T) -> T {
        if !self.is_height_safe(height) {
            self.to_insert
                .entry(Self::height_to_chunk_start(height))
                .or_default()
                .insert(height % HEIGHT_MAP_CHUNK_SIZE, value);
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
            .and_then(|map| map.get(&(height - chunk_start)).cloned())
            .or_else(|| {
                self.imported
                    .get(&chunk_start)
                    .and_then(|serialized| serialized.map.get(height - chunk_start))
                    .cloned()
            })
    }

    #[inline(always)]
    pub fn is_height_safe(&self, height: usize) -> bool {
        self.initial_first_unsafe_height.unwrap_or(0) > height
    }

    fn read_dir(&self) -> BTreeMap<usize, PathBuf> {
        Self::_read_dir(&self.path_all, &self.serialization)
    }

    pub fn _read_dir(path: &str, serialization: &Serialization) -> BTreeMap<usize, PathBuf> {
        fs::read_dir(path)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                let extension = path.extension().unwrap().to_str().unwrap();

                path.is_file() && extension == serialization.to_extension()
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

    fn import(&self, path: &Path) -> color_eyre::Result<SerializedHeightMap<T>> {
        self.serialization
            .import::<SerializedHeightMap<T>>(path.to_str().unwrap())
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

    fn path_last(&self) -> &Option<String> {
        &self.path_last
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
        self.to_insert
            .iter_mut()
            .enumerate()
            .for_each(|(_, (chunk_start, map))| {
                let serialized =
                    self.imported
                        .entry(chunk_start.to_owned())
                        .or_insert(SerializedHeightMap {
                            version: self.version,
                            map: vec![],
                        });

                mem::take(map)
                    .into_iter()
                    .for_each(|(chunk_height, value)| {
                        match serialized.map.len().cmp(&chunk_height) {
                            Ordering::Greater => serialized.map[chunk_height] = value,
                            Ordering::Equal => serialized.map.push(value),
                            Ordering::Less => panic!(),
                        }
                    });
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

                let serialized = self.imported.get(chunk_start).unwrap_or_else(|| {
                    dbg!(&self.path_all, chunk_start, &self.imported);
                    panic!();
                });

                self.serialization.export(&path, serialized)?;

                if index == len - 1 {
                    if let Some(path_last) = self.path_last.as_ref() {
                        self.serialization
                            .export(path_last, serialized.map.last().unwrap())?;
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
            .map(|previous_height| {
                source.get(&previous_height).unwrap_or_else(|| {
                    dbg!(&self.path_all, &source.path_all, previous_height);
                    panic!()
                })
            })
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
            .map(|height| {
                source.get(&height).unwrap_or_else(|| {
                    dbg!(&self.path_all, &source.path_all, offset);
                    panic!();
                })
            })
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
                    .map(|height| {
                        OrderedFloat(source.get(&height).unwrap_or_else(|| {
                            dbg!(height, &source.path_all, size);
                            panic!()
                        }))
                    })
                    .collect_vec();

                vec.sort_unstable();

                if even {
                    (vec.get(median_index)
                        .unwrap_or_else(|| {
                            dbg!(median_index, &self.path_all, &source.path_all, size);
                            panic!()
                        })
                        .0
                        + vec.get(median_index - 1).unwrap().0)
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
