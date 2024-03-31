use crate::datasets::{ProcessedBlockData, ProcessedDateData};

use super::AnyDataset;

pub trait GenericDataset: AnyDataset {
    fn insert_block_data(&self, _: &ProcessedBlockData) {}

    fn insert_date_data(&self, _: &ProcessedDateData) {}
}
