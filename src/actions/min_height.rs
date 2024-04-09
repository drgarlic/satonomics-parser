use crate::{
    databases::Databases,
    datasets::{AllDatasets, AnyDatasets},
    states::States,
};

pub fn find_first_unsafe_height(
    states: &mut States,
    databases: &mut Databases,
    datasets: &AllDatasets,
) -> usize {
    let min_initial_last_address_date = datasets.address.get_min_initial_state().last_date.lock();

    let min_initial_last_address_height =
        datasets.address.get_min_initial_state().last_height.lock();

    states
        .date_data_vec
        .iter()
        .last()
        .map(|date_data| date_data.date)
        .and_then(|last_safe_date| {
            let min_datasets_last_height = datasets.get_min_initial_state().last_height.lock();
            let min_datasets_last_date = datasets.get_min_initial_state().last_date.lock();

            println!("min_datasets_last_height: {:?}", min_datasets_last_height);
            println!("min_datasets_last_date: {:?}", min_datasets_last_date);

            if min_datasets_last_date.map_or(true, |min_datasets_last_date| min_datasets_last_date < *last_safe_date) {
                return None;
            }

            datasets
                .date_metadata
                .last_height
                .get(&last_safe_date)
                .and_then(|last_safe_height| {
                    if min_datasets_last_height.map_or(true, |min_datasets_last_height| min_datasets_last_height < last_safe_height) {
                        println!("last_safe_height ({last_safe_height}) > min_datasets_height ({min_datasets_last_height:?})");

                        None
                    } else {
                        Some(last_safe_height + 1)
                    }
                }
            )
        })
        .unwrap_or_else(|| {
            println!("Starting over...");

            states.reset();

            let include_addresses = min_initial_last_address_date.is_none() || min_initial_last_address_height.is_none();

            databases.reset(include_addresses);

            0
        })
}
