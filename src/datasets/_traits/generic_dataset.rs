use crate::datasets::ProcessedBlockData;

use super::AnyDataset;

pub trait GenericDataset: AnyDataset {
    fn insert_data(&self, _: &ProcessedBlockData) {}
}
