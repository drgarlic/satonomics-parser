use crate::{
    databases::Databases,
    datasets::{AllDatasets, AnyDateDatasets, AnyHeightDatasets},
    states::States,
};

pub fn min_height(states: &mut States, databases: &Databases, datasets: &AllDatasets) -> usize {
    states
        // Our reference
        .date_data_vec
        .iter()
        .map(|date_data| date_data.date)
        .max()
        .and_then(|last_state_date| {
            let min_datasets_height = datasets.height.get_min_last_height();
            let min_datasets_date = datasets.date.get_min_last_date();

            if min_datasets_date.map_or(true, |min_datasets_date| min_datasets_date < *last_state_date) {
                return None;
            }

            datasets.date.get_date_to_last_height().get(&last_state_date.to_string()).and_then(|state_last_height| {
                if min_datasets_height.unwrap_or(0) < *state_last_height {
                    println!("state_last_height ({state_last_height}) > min_datasets_height ({min_datasets_height:?})");

                     None
                } else {
                    // No need to recompute the last height if everything's good
                    Some(*state_last_height + 1)
                }
            })
        })
        .unwrap_or_else(|| {
            println!("Starting over...");

            let _ = states.reset();

            // TODO: Clear address stuff only if needed
            let _ = databases.reset(true);

            0
        })
}
