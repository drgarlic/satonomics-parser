use chrono::NaiveDate;

use super::{AnyDataset, AnyDatasets};

#[derive(Default, Debug)]
pub struct MinInitialState {
    pub first_unsafe_date: Option<NaiveDate>,
    pub first_unsafe_height: Option<usize>,
    pub last_date: Option<NaiveDate>,
    pub last_height: Option<usize>,
}

impl MinInitialState {
    pub fn consume(&mut self, other: Self) {
        self.first_unsafe_date = other.first_unsafe_date;
        self.first_unsafe_height = other.first_unsafe_height;
        self.last_date = other.last_date;
        self.last_height = other.last_height;
    }

    pub fn compute_from_datasets(datasets: &dyn AnyDatasets) -> Self {
        Self {
            first_unsafe_date: Self::compute_min_initial_first_unsafe_date_from_datasets(datasets),
            first_unsafe_height: Self::compute_min_initial_first_unsafe_height_from_datasets(
                datasets,
            ),
            last_date: Self::compute_min_initial_last_date_from_datasets(datasets),
            last_height: Self::compute_min_initial_last_height_from_datasets(datasets),
        }
    }

    fn compute_min_initial_last_date_from_datasets(
        datasets: &dyn AnyDatasets,
    ) -> Option<NaiveDate> {
        datasets
            .to_any_dataset_vec()
            .into_iter()
            .filter(|dataset| !dataset.to_any_inserted_date_map_vec().is_empty())
            .map(|dataset| dataset.get_min_initial_state().last_date.as_ref().cloned())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_last_height_from_datasets(datasets: &dyn AnyDatasets) -> Option<usize> {
        datasets
            .to_any_dataset_vec()
            .into_iter()
            .filter(|dataset| !dataset.to_any_inserted_height_map_vec().is_empty())
            .map(|dataset| {
                dataset
                    .get_min_initial_state()
                    .last_height
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
            .to_any_dataset_vec()
            .into_iter()
            .filter(|dataset| !dataset.to_any_inserted_date_map_vec().is_empty())
            .map(|dataset| {
                dataset
                    .get_min_initial_state()
                    .first_unsafe_date
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
            .to_any_dataset_vec()
            .into_iter()
            .filter(|dataset| !dataset.to_any_inserted_height_map_vec().is_empty())
            .map(|dataset| {
                dataset
                    .get_min_initial_state()
                    .first_unsafe_height
                    .as_ref()
                    .cloned()
            })
            .min()
            .and_then(|opt| opt)
    }

    pub fn compute_from_dataset(dataset: &dyn AnyDataset) -> Self {
        Self {
            first_unsafe_date: Self::compute_min_initial_first_unsafe_date_from_dataset(dataset),
            first_unsafe_height: Self::compute_min_initial_first_unsafe_height_from_dataset(
                dataset,
            ),
            last_date: Self::compute_min_initial_last_date_from_dataset(dataset),
            last_height: Self::compute_min_initial_last_height_from_dataset(dataset),
        }
    }

    fn compute_min_initial_last_date_from_dataset(dataset: &dyn AnyDataset) -> Option<NaiveDate> {
        dataset
            .to_any_inserted_date_map_vec()
            .into_iter()
            .map(|map| map.get_initial_last_date())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_last_height_from_dataset(dataset: &dyn AnyDataset) -> Option<usize> {
        dataset
            .to_any_inserted_height_map_vec()
            .into_iter()
            .map(|map| map.get_initial_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_first_unsafe_date_from_dataset(
        dataset: &dyn AnyDataset,
    ) -> Option<NaiveDate> {
        dataset
            .to_any_inserted_date_map_vec()
            .into_iter()
            .map(|map| map.get_initial_first_unsafe_date())
            .min()
            .and_then(|opt| opt)
    }

    fn compute_min_initial_first_unsafe_height_from_dataset(
        dataset: &dyn AnyDataset,
    ) -> Option<usize> {
        dataset
            .to_any_inserted_height_map_vec()
            .into_iter()
            .map(|map| map.get_initial_first_unsafe_height())
            .min()
            .and_then(|opt| opt)
    }
}
