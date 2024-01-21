use chrono::Days;

use crate::structs::DateMap;

use super::{Databases, HeightDatasets, HeightDatasetsTrait, States};

pub fn min_height(
    states: &mut States,
    databases: &Databases,
    datasets: &HeightDatasets,
    date_to_first_block: &DateMap<usize>,
) -> usize {
    let max_date = states
        .date_data_vec
        .iter()
        .map(|date_data| date_data.date)
        .max();

    max_date
        .and_then(|date| date.checked_add_days(Days::new(1)))
        .and_then(|date| {
            let min_last_height = datasets.get_min_last_height();

            date_to_first_block.get(&date).map(|snapshot_start_height| {
                if min_last_height.unwrap_or(0) < snapshot_start_height - 1 {
                    println!("snapshot_start_height {snapshot_start_height} > last_saved_height {min_last_height:?}");

                    println!("Starting over...");
                    states.clear();
                    databases.clear().unwrap();

                     0
                } else {
                    snapshot_start_height
                }
            })
        })
        .unwrap_or(0)
}
