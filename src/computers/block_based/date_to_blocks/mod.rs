use itertools::Itertools;

use crate::{
    bitcoin::{create_group_blocks_by_day_closure, BitcoinDB},
    utils::timestamp_to_naive_date,
};

pub mod structs;

pub use self::structs::DateToBlocks;

pub fn compute_date_to_blocks(
    db: &BitcoinDB,
    block_count: usize,
) -> color_eyre::Result<DateToBlocks> {
    println!("Computing date_to_blocks...");

    let date_to_blocks = DateToBlocks::import()?;

    let first_unsafe_date_opt = date_to_blocks.get_min_first_unsafe_date();

    let mut first_block = first_unsafe_date_opt
        .and_then(|first_unsafe_date| date_to_blocks.date_to_first_block.get(&first_unsafe_date))
        .unwrap_or(0);

    db.iter_block(first_block, block_count)
        .batching(create_group_blocks_by_day_closure())
        .for_each(|blocks| {
            let date = timestamp_to_naive_date(blocks.first().unwrap().header.time);

            println!("Date: {date}");

            date_to_blocks.insert(&date, first_block, blocks.len());

            first_block += blocks.len();
        });

    date_to_blocks.export()?;

    Ok(date_to_blocks)
}
