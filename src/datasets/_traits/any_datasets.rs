use crate::datasets::ProcessedBlockData;

use super::{GenericDataset, MinInitialState};

pub trait AnyDatasets {
    fn get_min_initial_state(&self) -> &MinInitialState;

    fn to_generic_dataset_vec(&self) -> Vec<&(dyn GenericDataset + Send + Sync)>;

    fn insert_data(&self, processed_block_data: ProcessedBlockData) {
        let ProcessedBlockData { height, date, .. } = processed_block_data;

        self.to_generic_dataset_vec()
            .into_iter()
            .filter(|dataset| dataset.should_insert(height, date))
            .for_each(|dataset| dataset.insert_data(&processed_block_data));
    }
}
