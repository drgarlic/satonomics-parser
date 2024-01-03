use chrono::NaiveDate;

use crate::{
    bitcoin::BitcoinDB,
    structs::{AnyHeightMap, HeightMap},
    utils::timestamp_to_naive_date,
};

pub fn compute_height_to_date(
    db: &BitcoinDB,
    block_count: usize,
) -> color_eyre::Result<HeightMap<NaiveDate>> {
    println!("Computing height_to_date...");

    let height_to_date = HeightMap::new("height_to_date.json");

    let start = height_to_date.get_first_unsafe_height().unwrap_or(0);

    db.iter_block(start, block_count)
        .enumerate()
        .for_each(|(index, block)| {
            let height = start + index;
            println!("Height: {height}");

            height_to_date.insert(height, timestamp_to_naive_date(block.header.time));
        });

    height_to_date.export()?;

    Ok(height_to_date)
}
