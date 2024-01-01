use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::offset::Local;
use itertools::Itertools;

use crate::{
    structs::DateMap,
    utils::{create_group_blocks_by_day_closure, timestamp_to_naive_date},
};

pub mod export;
pub mod parse;

use export::*;
use parse::*;

pub fn compute_address_based_datasets(
    db: &BitcoinDB,
    block_count: usize,
    height_to_price: &[f32],
    date_to_first_block: &DateMap<usize>,
) -> color_eyre::Result<()> {
    println!("{:?} - Starting address based", Local::now());

    db.iter_block::<FBlock>(0, block_count)
        .batching(create_group_blocks_by_day_closure())
        .try_for_each(|blocks| -> color_eyre::Result<()> {
            let date = timestamp_to_naive_date(blocks.first().unwrap().header.time);

            let blocks_len = blocks.len();

            println!(
                "{:?} - Processing {date} ({} blocks)...",
                Local::now(),
                blocks_len + 1
            );

            blocks.into_iter().for_each(|block| {
                block.txdata.into_iter().for_each(|tx| {
                    tx.output
                        .into_iter()
                        .enumerate()
                        .for_each(|(txout_index, txout)| {
                            if txout.addresses.len() > 1 {
                                panic!("!!");
                            }
                        });

                    // tx.input.into_iter().for_each(|txin| txin.previous_output.);
                });
            });

            Ok(())
        })?;

    Ok(())
}
