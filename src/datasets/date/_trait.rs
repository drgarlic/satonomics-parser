use chrono::NaiveDate;
use rayon::prelude::*;

use crate::structs::AnyDateMap;

use super::ProcessedDateData;

pub trait AnyDateDataset {
    fn get_min_last_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|map| map.get_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_date())
            .min()
            .and_then(|opt| opt)
    }

    fn check_if_up_to_date(&self, date: NaiveDate) -> bool {
        self.get_min_initial_first_unsafe_date()
            .map_or(true, |min_initial_first_unsafe_date| {
                min_initial_first_unsafe_date <= date
            })
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_date_map_vec()
            .iter()
            .try_for_each(|map| map.export())
    }

    fn insert(&self, processed_date_data: &ProcessedDateData);

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)>;
}

pub trait AnyDateDatasets {
    fn get_min_last_date(&self) -> Option<NaiveDate> {
        self.to_any_date_map_vec()
            .iter()
            .map(|dataset| dataset.get_min_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn export_if_needed(&self, date: NaiveDate) -> color_eyre::Result<()> {
        self.to_any_date_map_vec()
            .par_iter()
            .filter(|dataset| dataset.check_if_up_to_date(date))
            .try_for_each(|dataset| dataset.export())
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_any_date_map_vec()
            .par_iter()
            .try_for_each(|dataset| dataset.export())
    }

    fn insert(&self, processed_date_data: ProcessedDateData) {
        let ProcessedDateData { date, .. } = processed_date_data;

        self.to_any_date_map_vec()
            .iter()
            .filter(|dataset| dataset.check_if_up_to_date(date))
            .for_each(|dataset| dataset.insert(&processed_date_data));
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateDataset + Send + Sync)>;
}
