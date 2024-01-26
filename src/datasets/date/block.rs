use std::fs;

use crate::structs::{AnyDateMap, DateMap};

use super::{AnyDateDataset, ProcessedDateData};

pub struct BlockDataset {
    pub first_height: DateMap<usize>,
    pub last_height: DateMap<usize>,
    pub block_count: DateMap<usize>,
}

impl BlockDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/block");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}.json");

        Ok(Self {
            first_height: DateMap::new(&f("first_height")),
            last_height: DateMap::new(&f("last_height")),
            block_count: DateMap::new(&f("block_count")),
        })
    }
}

impl AnyDateDataset for BlockDataset {
    fn insert(
        &self,
        ProcessedDateData {
            date,
            first_height,
            block_count,
            ..
        }: &ProcessedDateData,
    ) {
        self.first_height
            .insert(date.to_owned(), first_height.to_owned());

        self.last_height
            .insert(date.to_owned(), first_height + (block_count - 1).max(0));

        self.block_count
            .insert(date.to_owned(), block_count.to_owned());
    }

    fn to_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        let vec: Vec<&(dyn AnyDateMap + Send + Sync)> =
            vec![&self.block_count, &self.first_height, &self.last_height];

        vec
    }
}
