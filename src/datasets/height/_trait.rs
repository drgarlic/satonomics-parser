use rayon::prelude::*;

use crate::structs::AnyHeightMap;

use super::ProcessedBlockData;

pub trait AnyHeightDataset {
    fn get_min_last_height(&self) -> Option<usize> {
        self.to_vec()
            .iter()
            .map(|map| map.get_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_height(&self) -> Option<usize> {
        self.to_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_height())
            .min()
            .and_then(|opt| opt)
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_vec().iter().try_for_each(|map| map.export())
    }

    fn insert(&self, processed_block_data: &ProcessedBlockData);

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)>;
}

pub trait AnyHeightDatasets {
    fn get_min_last_height(&self) -> Option<usize> {
        self.to_vec()
            .iter()
            .map(|dataset| dataset.get_min_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn export_if_needed(&self, height: usize) -> color_eyre::Result<()> {
        self.to_vec()
            .par_iter()
            .filter(|dataset| Self::check_if_dataset_is_up_to_height(**dataset, height))
            .try_for_each(|dataset| dataset.export())
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_vec()
            .par_iter()
            .try_for_each(|dataset| dataset.export())
    }

    fn insert(&self, processed_block_data: ProcessedBlockData) {
        let ProcessedBlockData { height, .. } = processed_block_data;

        self.to_vec()
            .par_iter()
            .filter(|dataset| Self::check_if_dataset_is_up_to_height(**dataset, height))
            .for_each(|dataset| dataset.insert(&processed_block_data));
    }

    fn check_if_dataset_is_up_to_height(
        dataset: &(dyn AnyHeightDataset + Send + Sync),
        height: usize,
    ) -> bool {
        dataset.get_min_initial_first_unsafe_height().unwrap_or(0) <= height
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightDataset + Send + Sync)>;
}
