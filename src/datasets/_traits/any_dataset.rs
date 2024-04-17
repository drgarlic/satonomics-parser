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

    fn to_any_map_vec(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        let heights = self
            .to_any_height_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        let dates = self
            .to_any_date_map_vec()
            .into_iter()
            .map(|d| d.as_any_map());

        let bis = self
            .to_any_bi_map_vec()
            .into_iter()
            .flat_map(|d| d.as_any_map());

        heights.chain(dates).chain(bis).collect_vec()
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![]
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![]
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![]
    }

    fn to_any_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![]
    }

    fn to_any_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![]
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        let mut vec = self.to_any_height_map_vec();

        vec.append(
            &mut self
                .to_any_bi_map_vec()
                .iter()
                .map(|bi| bi.get_height())
                .collect_vec(),
        );

        vec
    }

    fn to_any_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        let mut vec = self.to_any_date_map_vec();

        vec.append(
            &mut self
                .to_any_bi_map_vec()
                .iter()
                .map(|bi| bi.get_date())
                .collect_vec(),
        );

        vec
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.to_any_map_vec().is_empty()
    }

    fn pre_export(&mut self) {
        self.to_any_mut_height_map_vec()
            .into_iter()
            .for_each(|d| d.pre_export());

        self.to_any_mut_date_map_vec()
            .into_iter()
            .for_each(|d| d.pre_export());

        self.to_any_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.pre_export())
        });
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_map_vec()
            .into_par_iter()
            .try_for_each(|map| -> color_eyre::Result<()> { map.export() })
    }

    fn post_export(&mut self) {
        self.to_any_mut_height_map_vec()
            .into_iter()
            .for_each(|d| d.post_export());

        self.to_any_mut_date_map_vec()
            .into_iter()
            .for_each(|d| d.post_export());

        self.to_any_mut_bi_map_vec().into_iter().for_each(|d| {
            d.as_any_mut_map()
                .into_iter()
                .for_each(|map| map.post_export())
        });
    }
}
