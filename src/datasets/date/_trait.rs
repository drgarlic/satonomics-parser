use chrono::NaiveDate;
use rayon::prelude::*;

use crate::structs::AnyDateMap;

use super::ProcessedDateData;

pub trait AnyDateDataset {
    fn get_min_last_date(&self) -> Option<NaiveDate> {
        self.to_vec()
            .iter()
            .map(|map| map.get_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn get_min_initial_first_unsafe_date(&self) -> Option<NaiveDate> {
        self.to_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_date())
            .min()
            .and_then(|opt| opt)
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_vec().iter().try_for_each(|map| map.export())
    }

    fn insert(&self, processed_block_data: &ProcessedDateData);

    fn to_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)>;
}

pub trait AnyDateDatasets {
    fn get_min_last_date(&self) -> Option<NaiveDate> {
        self.to_vec()
            .iter()
            .map(|dataset| dataset.get_min_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn export_if_needed(&self, date: NaiveDate) -> color_eyre::Result<()> {
        self.to_vec()
            .par_iter()
            .filter(|dataset| Self::check_if_dataset_is_up_to_date(**dataset, date))
            .try_for_each(|dataset| dataset.export())
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_vec()
            .par_iter()
            .try_for_each(|dataset| dataset.export())
    }

    fn insert(&self, processed_date_data: ProcessedDateData) {
        let ProcessedDateData { date, .. } = processed_date_data;

        self.to_vec()
            .iter()
            .filter(|dataset| Self::check_if_dataset_is_up_to_date(**dataset, date))
            .for_each(|dataset| dataset.insert(&processed_date_data));
    }

    // TODO: Move to DateMap do the same for HeightMap
    fn check_if_dataset_is_up_to_date(
        dataset: &(dyn AnyDateDataset + Send + Sync),
        date: NaiveDate,
    ) -> bool {
        dataset
            .get_min_initial_first_unsafe_date()
            .map_or(true, |min_initial_first_unsafe_date| {
                min_initial_first_unsafe_date <= date
            })
    }

    fn to_vec(&self) -> Vec<&(dyn AnyDateDataset + Send + Sync)>;
}
