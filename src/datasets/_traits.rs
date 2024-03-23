use chrono::NaiveDate;
use rayon::prelude::*;

use crate::parse::{AnyDateMap, AnyHeightMap};

use super::{ProcessedBlockData, ProcessedDateData};

pub trait AnyDataset2 {
    fn get_min_last_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|map| map.get_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_last_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|map| map.get_initial_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate>;

    fn get_min_initial_last_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|map| map.get_initial_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_last_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|map| map.get_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize>;

    fn compute_min_initial_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_date())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_first_unsafe_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_height())
            .min()
            .and_then(|opt| opt)
    }

    #[inline(always)]
    fn process_height(&self, height: usize) -> bool {
        !self.to_any_height_map_vec().is_empty()
            && self.get_min_initial_first_unsafe_height().unwrap_or(0) <= height
    }

    #[inline(always)]
    fn process_date(&self, date: NaiveDate) -> bool {
        !self.to_any_date_map_vec().is_empty()
            && self
                .get_min_initial_first_unsafe_date()
                .map_or(true, |min_initial_first_unsafe_date| {
                    min_initial_first_unsafe_date <= date
                })
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_date_map_vec()
            .iter()
            .try_for_each(|map| map.export())?;

        self.to_any_height_map_vec()
            .iter()
            .try_for_each(|map| map.export())?;

        Ok(())
    }

    fn insert_block_data(&self, _: &ProcessedBlockData) {}

    fn insert_date_data(&self, _: &ProcessedDateData) {}

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![]
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![]
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.to_any_height_map_vec().is_empty() || self.to_any_date_map_vec().is_empty()
    }

    // fn name(&self) -> &str;
}

pub trait AnyDataset {
    fn get_min_last_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|map| map.get_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_last_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|map| map.get_initial_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate>;

    fn get_min_initial_last_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|map| map.get_initial_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_last_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|map| map.get_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize>;

    fn compute_min_initial_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_date())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_first_unsafe_height(&self) -> Option<usize> {
        self.to_any_height_map_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_height())
            .min()
            .and_then(|opt| opt)
    }

    #[inline(always)]
    fn process_height(&self, height: usize) -> bool {
        !self.to_any_height_map_vec().is_empty()
            && self.get_min_initial_first_unsafe_height().unwrap_or(0) <= height
    }

    #[inline(always)]
    fn process_date(&self, date: NaiveDate) -> bool {
        !self.to_any_date_map_vec().is_empty()
            && self
                .get_min_initial_first_unsafe_date()
                .map_or(true, |min_initial_first_unsafe_date| {
                    min_initial_first_unsafe_date <= date
                })
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_date_map_vec()
            .iter()
            .try_for_each(|map| map.export())?;

        self.to_any_height_map_vec()
            .iter()
            .try_for_each(|map| map.export())?;

        Ok(())
    }

    fn insert_block_data(&self, _: &ProcessedBlockData) {}

    fn insert_date_data(&self, _: &ProcessedDateData) {}

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![]
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![]
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.to_any_height_map_vec().is_empty() || self.to_any_date_map_vec().is_empty()
    }

    // fn name(&self) -> &str;
}

pub trait AnyDatasets {
    fn get_min_initial_last_date(&self) -> Option<NaiveDate> {
        self.to_any_dataset_vec()
            .iter()
            .filter(|dataset| !dataset.to_any_date_map_vec().is_empty())
            .map(|dataset| dataset.get_min_initial_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_last_height(&self) -> Option<usize> {
        self.to_any_dataset_vec()
            .iter()
            .filter(|dataset| !dataset.to_any_height_map_vec().is_empty())
            .map(|dataset| dataset.get_min_initial_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.to_any_dataset_vec()
            .iter()
            .filter(|dataset| !dataset.to_any_date_map_vec().is_empty())
            .map(|dataset| dataset.get_min_initial_first_unsafe_date())
            .min()
            .and_then(|opt| opt.to_owned())
    }

    fn get_min_initial_first_unsafe_height(&self) -> Option<usize> {
        self.to_any_dataset_vec()
            .iter()
            .filter(|dataset| !dataset.to_any_height_map_vec().is_empty())
            .map(|dataset| dataset.get_min_initial_first_unsafe_height())
            .min()
            .and_then(|opt| opt.to_owned())
    }

    fn insert_date_data(&self, processed_date_data: ProcessedDateData) {
        let ProcessedDateData { date, .. } = processed_date_data;

        self.to_any_dataset_vec()
            .par_iter()
            .filter(|dataset| dataset.process_date(date))
            .for_each(|dataset| dataset.insert_date_data(&processed_date_data));
    }

    fn insert_block_data(&self, processed_block_data: ProcessedBlockData) {
        let ProcessedBlockData { height, date, .. } = processed_block_data;

        self.to_any_dataset_vec()
            .par_iter()
            .filter(|dataset| dataset.process_height(height) || dataset.process_date(date))
            .for_each(|dataset| dataset.insert_block_data(&processed_block_data));
    }

    fn export_if_needed(&self, date: NaiveDate, height: usize) -> color_eyre::Result<()> {
        self.to_any_dataset_vec()
            .par_iter()
            .filter(|dataset| dataset.process_height(height) || dataset.process_date(date))
            .try_for_each(|dataset| -> color_eyre::Result<()> { dataset.export() })?;

        Ok(())
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_dataset_vec()
            .iter()
            .try_for_each(|dataset| dataset.export())?;

        Ok(())
    }

    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)>;

    fn name<'a>() -> &'a str;
}
