use chrono::NaiveDate;
use itertools::Itertools;
use rayon::prelude::*;

use crate::parse::{AnyBiMap, AnyDateMap, AnyHeightMap, AnyMap};

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
                .unwrap_or(0)
                <= height
    }

    #[inline(always)]
    fn should_insert_date(&self, date: NaiveDate) -> bool {
        !self.to_any_inserted_date_map_vec().is_empty()
            && self
                .get_min_initial_state()
                .first_unsafe_date
                .map_or(true, |min_initial_first_unsafe_date| {
                    min_initial_first_unsafe_date <= date
                })
    }

    fn to_any_inserted_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        // fn to_any_inserted_map_vec(&self) -> impl Iterator<Item = &(dyn AnyMap + Send + Sync)> {
        self.to_any_inserted_height_map_vec()
            .into_iter()
            .map(|map| map.as_any_map())
            .chain(
                self.to_any_inserted_date_map_vec()
                    .into_iter()
                    .map(|map| map.as_any_map()),
            )
            .collect_vec()
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

    fn to_any_exported_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        let heights = self
            .to_any_exported_height_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        let dates = self
            .to_any_exported_date_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        let bis = self
            .to_any_exported_bi_map_vec()
            .into_iter()
            .flat_map(|d| d.as_any_map());

        heights.chain(dates.chain(bis)).collect_vec()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.to_any_inserted_map_vec().is_empty()
    }

    // fn prepare(&self, _: &ExportData) {}

    // fn compute(&self, _: &ExportData) {}

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_exported_map_vec()
            .into_par_iter()
            .try_for_each(|map| -> color_eyre::Result<()> { map.export() })
    }

    fn clean(&self) {
        self.to_any_exported_map_vec()
            .into_par_iter()
            .for_each(|map| {
                map.clean();
            })
    }
}
