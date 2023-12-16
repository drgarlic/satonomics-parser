use std::path::Path;

use bitcoin_explorer::BitcoinDB;
// use chrono::NaiveDate;
// use itertools::Itertools;

use crate::{
    computers::{
        compute_address_based_datasets,
        // HeightToRewards,
        compute_date_to_blocks,
        compute_height_to_date,
        compute_height_to_price,
        // compute_height_to_rewards,
        compute_height_to_timestamp,
        // compute_date_to_price,
        // compute_utxo_based_datasets,
        DateToBlocks,
    },
    utils::{time, BITCOIN_DATADIR_RAW_PATH},
};

pub fn run() -> color_eyre::Result<usize> {
    let db = BitcoinDB::new(Path::new(BITCOIN_DATADIR_RAW_PATH), true)?;

    let block_count = db.get_block_count();
    println!("{block_count} blocks found.");

    time(|| compute_height_to_timestamp(&db, block_count))?;

    let _height_to_date = time(|| compute_height_to_date(&db, block_count))?.consume();

    let DateToBlocks {
        date_to_first_block,
        // date_to_last_block,
        ..
    } = time(|| compute_date_to_blocks(&db, block_count))?;

    let height_to_price = time(|| compute_height_to_price(&db, block_count))?.consume();

    // let _date_to_price = time(compute_date_to_price)?;

    // let HeightToRewards { .. } = time(|| compute_height_to_rewards(&db, block_count))?;

    // compute_utxo_based_datasets(&db, block_count, &height_to_price, &date_to_first_block)?;

    compute_address_based_datasets(&db, block_count, &height_to_price, &date_to_first_block)?;

    Ok(block_count)
}
