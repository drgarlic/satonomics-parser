use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use chrono::NaiveDate;
use itertools::Itertools;
use rayon::prelude::*;

use crate::utils::{export_snapshot, import_snapshot_vec};

use super::{BlockData, DateData, SerializedBlockData};

pub struct BlockDatasPerDay(Vec<DateData>);

const BLOCKS_DATAS_PER_DAY_SNAPSHOT_NAME: &str = "height_to_aged__block_datas_per_day";

impl BlockDatasPerDay {
    pub fn import(height_to_date: &[NaiveDate]) -> color_eyre::Result<Self> {
        let mut dates_set = HashSet::<&NaiveDate>::from_iter(height_to_date)
            .drain()
            .collect_vec();

        dates_set.sort();

        Ok(Self(
            import_snapshot_vec::<Vec<SerializedBlockData>>(
                BLOCKS_DATAS_PER_DAY_SNAPSHOT_NAME,
                true,
            )?
            .par_iter()
            .enumerate()
            .map(|(index, imported_date_data)| DateData {
                date: dates_set[index].to_owned(),
                blocks: imported_date_data.iter().map(BlockData::import).collect(),
            })
            .collect(),
        ))
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        let value = self
            .par_iter()
            .map(|date_data| {
                date_data
                    .blocks
                    .iter()
                    .map(|block_data| block_data.serialize())
                    .collect_vec()
            })
            .collect::<Vec<_>>();

        export_snapshot(BLOCKS_DATAS_PER_DAY_SNAPSHOT_NAME, &value, false)
    }

    pub fn last_block(&mut self) -> &mut BlockData {
        self.last_mut().unwrap().blocks.last_mut().unwrap()
    }
}

impl Deref for BlockDatasPerDay {
    type Target = Vec<DateData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BlockDatasPerDay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
