use chrono::NaiveDate;
use parking_lot::Mutex;

use super::{AnyDataset, AnyDatasets};

#[derive(Default)]
pub struct MinInitialState {
    pub first_unsafe_date: Mutex<Option<NaiveDate>>,
    pub first_unsafe_height: Mutex<Option<usize>>,
    pub last_date: Mutex<Option<NaiveDate>>,
    pub last_height: Mutex<Option<usize>>,
}

impl MinInitialState {
    pub fn compute_from_datasets(&self, datasets: &dyn AnyDatasets) {
        *self.first_unsafe_date.lock() =
            Self::compute_min_initial_first_unsafe_date_from_datasets(datasets);

        *self.first_unsafe_height.lock() =
            Self::compute_min_initial_first_unsafe_height_from_datasets(datasets);

        *self.last_date.lock() = Self::compute_min_initial_last_date_from_datasets(datasets);

        *self.last_height.lock() = Self::compute_min_initial_last_height_from_datasets(datasets);
    }

    fn compute_min_initial_last_date_from_datasets(
        datasets: &dyn AnyDatasets,
    ) -> Option<NaiveDate> {
        datasets
            .to_generic_dataset_vec()
            .iter()
            .filter(|dataset| !dataset.to_any_inserted_date_map_vec().is_empty())
            .map(|dataset| {
                dataset
                    .get_min_initial_state()
                    .last_date
                    .lock()
                    .as_ref()
                    .cloned()
            })
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_last_height_from_datasets(datasets: &dyn AnyDatasets) -> Option<usize> {
        datasets
            .to_generic_dataset_vec()
            .iter()
            .filter(|dataset| !dataset.to_any_inserted_height_map_vec().is_empty())
            .map(|dataset| {
                dataset
                    .get_min_initial_state()
                    .last_height
                    .lock()
                    .as_ref()
                    .cloned()
            })
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_first_unsafe_date_from_datasets(
        datasets: &dyn AnyDatasets,
    ) -> Option<NaiveDate> {
        datasets
            .to_generic_dataset_vec()
            .iter()
            .filter(|dataset| !dataset.to_any_inserted_date_map_vec().is_empty())
            .map(|dataset| {
                dataset
                    .get_min_initial_state()
                    .first_unsafe_date
                    .lock()
                    .as_ref()
                    .cloned()
            })
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_first_unsafe_height_from_datasets(
        datasets: &dyn AnyDatasets,
    ) -> Option<usize> {
        datasets
            .to_generic_dataset_vec()
            .iter()
            .filter(|dataset| !dataset.to_any_inserted_height_map_vec().is_empty())
            .map(|dataset| {
                dataset
                    .get_min_initial_state()
                    .first_unsafe_height
                    .lock()
                    .as_ref()
                    .cloned()
            })
            .min()
            .and_then(|opt| opt)
    }

    pub fn compute_from_dataset(&self, dataset: &dyn AnyDataset) {
        *self.first_unsafe_date.lock() =
            Self::compute_min_initial_first_unsafe_date_from_dataset(dataset);

        *self.first_unsafe_height.lock() =
            Self::compute_min_initial_first_unsafe_height_from_dataset(dataset);

        *self.last_date.lock() = Self::compute_min_initial_last_date_from_dataset(dataset);

        *self.last_height.lock() = Self::compute_min_initial_last_height_from_dataset(dataset);
    }

    fn compute_min_initial_last_date_from_dataset(dataset: &dyn AnyDataset) -> Option<NaiveDate> {
        dataset
            .to_any_inserted_date_map_vec()
            .iter()
            .map(|map| map.get_initial_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_last_height_from_dataset(dataset: &dyn AnyDataset) -> Option<usize> {
        dataset
            .to_any_inserted_height_map_vec()
            .iter()
            .map(|map| map.get_initial_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_first_unsafe_date_from_dataset(
        dataset: &dyn AnyDataset,
    ) -> Option<NaiveDate> {
        dataset
            .to_any_inserted_date_map_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_date())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_first_unsafe_height_from_dataset(
        dataset: &dyn AnyDataset,
    ) -> Option<usize> {
        dataset
            .to_any_inserted_height_map_vec()
            .iter()
            .map(|map| map.get_initial_first_unsafe_height())
            .min()
            .and_then(|opt| opt)
    }
}
