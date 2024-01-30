use rayon::prelude::*;

use crate::structs::AnyHeightMap;

use super::ProcessedBlockData;

pub trait AnyHeightDataset {
    fn get_min_last_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|map| map.get_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_height())
            .min()
            .and_then(|opt| opt)
    }

    fn check_if_up_to_height(&self, height: usize) -> bool {
        self.get_min_initial_first_unsafe_height().unwrap_or(0) <= height
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_height_map_vec()
            .iter()
            .try_for_each(|map| map.export())
    }

    fn insert(&self, processed_block_data: &ProcessedBlockData);

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)>;
}

pub trait AnyHeightDatasets {
    fn get_min_last_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|dataset| dataset.get_min_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn insert(&self, processed_block_data: ProcessedBlockData) {
        let ProcessedBlockData { height, .. } = processed_block_data;

        self.to_any_height_map_vec()
            .par_iter()
            .filter(|dataset| dataset.check_if_up_to_height(height))
            .for_each(|dataset| dataset.insert(&processed_block_data));
    }

    fn export_if_needed(&self, height: usize) -> color_eyre::Result<()> {
        self.to_any_height_map_vec()
            .par_iter()
            .filter(|dataset| dataset.check_if_up_to_height(height))
            .try_for_each(|dataset| dataset.export())
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_height_map_vec()
            .par_iter()
            .try_for_each(|dataset| dataset.export())
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightDataset + Send + Sync)>;
}
