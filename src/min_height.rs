use chrono::NaiveDate;

use crate::{
    databases::Databases,
    datasets::{AllDatasets, AnyDatasets},
    states::States,
};

pub fn find_first_unsafe_height(
    states: &mut States,
    databases: &Databases,
    datasets: &AllDatasets,
    min_initial_last_address_date: &Option<NaiveDate>,
    min_initial_last_address_height: &Option<usize>,
) -> usize {
    states
        .date_data_vec
        .iter()
        .last()
        .map(|date_data| date_data.date)
        .and_then(|last_safe_date| {
            let min_datasets_last_height = datasets.get_min_initial_last_height();
            let min_datasets_last_date = datasets.get_min_initial_last_date();

            if min_datasets_last_date.map_or(true, |min_datasets_last_date| min_datasets_last_date < *last_safe_date) {
                return None;
            }

            datasets.get_date_to_last_height().get(&last_safe_date.to_string()).and_then(|last_safe_height| {
                if min_datasets_last_height.map_or(true, |min_datasets_last_height| min_datasets_last_height < *last_safe_height) {
                    println!("last_safe_height ({last_safe_height}) > min_datasets_height ({min_datasets_last_height:?})");

                     None
                } else {
                    Some(*last_safe_height + 1)
                }
            })
        })
        .unwrap_or_else(|| {
            println!("Starting over...");

            states.reset();

            databases.reset(min_initial_last_address_date.is_none() || min_initial_last_address_height.is_none());

            0
        })
}
