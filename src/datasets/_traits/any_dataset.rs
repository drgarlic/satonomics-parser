use chrono::NaiveDate;
use rayon::prelude::*;

use crate::{
    datasets::ExportData,
    parse::{AnyBiMap, AnyDateMap, AnyHeightMap},
};

use super::MinInitialState;

pub trait AnyDataset {
    fn get_min_initial_state(&self) -> &MinInitialState;

    fn should_insert(&self, height: usize, date: NaiveDate) -> bool {
        self.should_insert_height(height) || self.should_insert_date(date)
    }

    #[inline(always)]
    fn should_insert_height(&self, height: usize) -> bool {
        !self.to_any_inserted_height_map_vec().is_empty()
            && self
                .get_min_initial_state()
                .first_unsafe_height
                .lock()
                .unwrap_or(0)
                <= height
    }

    #[inline(always)]
    fn should_insert_date(&self, date: NaiveDate) -> bool {
        !self.to_any_inserted_date_map_vec().is_empty()
            && self
                .get_min_initial_state()
                .first_unsafe_date
                .lock()
                .map_or(true, |min_initial_first_unsafe_date| {
                    min_initial_first_unsafe_date <= date
                })
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_inserted_date_map_vec()
            .par_iter()
            .try_for_each(|map| map.export_then_clean())?;

        self.to_any_inserted_height_map_vec()
            .par_iter()
            .try_for_each(|map| map.export_then_clean())?;

        Ok(())
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![]
    }

    fn to_any_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![]
    }

    fn to_any_exported_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![]
    }

    fn to_any_exported_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![]
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.to_any_inserted_height_map_vec().is_empty()
            || self.to_any_inserted_date_map_vec().is_empty()
    }

    fn prepare(&self) {
        self.to_any_inserted_height_map_vec()
            .iter()
            .for_each(|map| map.prepare_tmp_data());

        self.to_any_inserted_date_map_vec()
            .iter()
            .for_each(|map| map.prepare_tmp_data())
    }

    fn compute(&mut self, _: &ExportData) {}

    fn export_then_clean(&mut self) -> color_eyre::Result<()> {
        self.to_any_exported_bi_map_vec()
            .par_iter()
            .try_for_each(|map| -> color_eyre::Result<()> { map.export_then_clean() })?;

        self.to_any_exported_height_map_vec()
            .par_iter()
            .try_for_each(|map| -> color_eyre::Result<()> { map.export_then_clean() })?;

        self.to_any_exported_date_map_vec()
            .par_iter()
            .try_for_each(|map| -> color_eyre::Result<()> { map.export_then_clean() })?;

        Ok(())
    }
}
