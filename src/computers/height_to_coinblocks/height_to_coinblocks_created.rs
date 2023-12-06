use itertools::Itertools;
use std::{collections::HashMap, path::Path, time::Instant};

use crate::{
    consts::OUTPUTS_FOLDER_RAW_PATH,
    utils::{export_json, import_json_map},
};

pub fn compute_height_to_coinblocks_created_map(
    height_to_coinbase_map: HashMap<String, f64>,
) -> color_eyre::Result<HashMap<String, f64>> {
    println!("Computing height_to_coinblocks_created...");
    let time = Instant::now();

    let path = Path::new(OUTPUTS_FOLDER_RAW_PATH).join("height_to_coinblocks_created.json");

    let mut map: HashMap<String, f64> = import_json_map(path.as_path(), true)?;

    let map_metadata = get_metadata_from_heights_map(&map);

    let start = map_metadata.first_unsafe_height.unwrap_or(0);

    let mut coinblocks_created = map_metadata.last_safe_value.unwrap_or(0.0);

    height_to_coinbase_map
        .iter()
        .map(|(height, coinbase)| (height.parse::<usize>().unwrap(), *coinbase))
        .sorted_by_key(|(height, _)| *height)
        .skip(start)
        .for_each(|(height, coinbase)| {
            coinblocks_created += coinbase;

            map.insert(height.to_string(), coinblocks_created);
        });

    export_json(&path, &map)?;

    println!("Took {} seconds\n", time.elapsed().as_secs_f32());
    Ok(map)
}
