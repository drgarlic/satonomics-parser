use bitcoin_explorer::{BitcoinDB, SBlock};

use crate::structs::HeightMap;

pub fn compute_height_to_timestamp(
    db: &BitcoinDB,
    block_count: usize,
) -> color_eyre::Result<HeightMap<u32>> {
    println!("Computing height_to_timestamp...");

    let height_to_timestamp = HeightMap::new("height_to_timestamp.json");

    let start = height_to_timestamp.get_first_unsafe_height().unwrap_or(0);

    db.iter_block::<SBlock>(start, block_count)
        .enumerate()
        .for_each(|(index, block)| {
            let height = start + index;
            println!("Height: {height}");

            height_to_timestamp.insert(height, block.header.time);
        });

    height_to_timestamp.export()?;

    Ok(height_to_timestamp)
}
