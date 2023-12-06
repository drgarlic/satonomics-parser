use std::{
    cell::RefCell,
    collections::HashSet,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use chrono::NaiveDate;
use itertools::Itertools;

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
            .drain(..)
            .enumerate()
            .map(|(index, mut imported_date_data)| DateData {
                date: dates_set[index].to_owned(),
                blocks: RefCell::new(
                    imported_date_data
                        .drain(..)
                        .map(|serialized_block_data| {
                            Rc::new(BlockData::import(serialized_block_data))
                        })
                        .collect_vec(),
                ),
            })
            .collect_vec(),
        ))
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        let value = self
            .iter()
            .map(|date_data| {
                date_data
                    .blocks
                    .borrow()
                    .iter()
                    .map(|block_data| {
                        (
                            block_data.price,
                            block_data.txid_index_to_outputs.to_owned(),
                        )
                    })
                    .collect_vec()
            })
            .collect_vec();

        export_snapshot(BLOCKS_DATAS_PER_DAY_SNAPSHOT_NAME, &value, false)?;

        Ok(())
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
