use bitcoin_explorer::{BitcoinDB, FBlock};
use std::{collections::HashMap, path::Path, time::Instant};

use crate::{
    consts::OUTPUTS_FOLDER_RAW_PATH,
    utils::{convert_sats_to_bitcoins, export_json, import_json_map},
};

pub fn compute_height_to_coinblocks_destroyed_map(
    db: &BitcoinDB,
    block_count: usize,
) -> color_eyre::Result<HashMap<String, f64>> {
    println!("Computing height_to_coinblocks_destroyed...");
    let time = Instant::now();

    let path = Path::new(OUTPUTS_FOLDER_RAW_PATH).join("height_to_coinblocks_destroyed.json");

    let mut map: HashMap<String, f64> = import_json_map(path.as_path(), true)?;

    let map_metadata = get_metadata_from_heights_map(&map);

    let start = map_metadata.first_unsafe_height.unwrap_or(0);

    db.iter_block::<FBlock>(start, block_count)
        .enumerate()
        .try_for_each(|(index, block)| -> color_eyre::Result<()> {
            let height = start + index;
            println!("Height: {height}");

            let coinblocks_destroyed: f64 = block
                .txdata
                .iter()
                .map(|tx| {
                    tx.output
                        .iter()
                        .map(|txout| {
                            let tx_height = db.get_height_of_transaction(&tx.txid).unwrap();

                            convert_sats_to_bitcoins(txout.value) * ((height - tx_height) as f64)
                        })
                        .sum::<f64>()
                })
                .sum();

            map.insert(height.to_string(), coinblocks_destroyed);

            Ok(())
        })?;

    export_json(&path, &map)?;

    println!("Took {} seconds\n", time.elapsed().as_secs_f32());
    Ok(map)
}
