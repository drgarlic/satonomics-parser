use std::path::Path;

mod bitcoin;
mod databases;
mod datasets;
mod export_all;
mod iter_blocks;
mod min_height;
mod parse_block;
mod states;
mod structs;
mod utils;

use crate::{bitcoin::BitcoinDB, iter_blocks::iter_blocks, structs::BITCOIN_DATADIR_RAW_PATH};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Daemon::start();
    //
    // loop {
    //     Daemon::wait_sync()?;
    //
    //     Daemon::stop();
    //
    //
    let bitcoin_db = BitcoinDB::new(Path::new(BITCOIN_DATADIR_RAW_PATH), true)?;

    let block_count = bitcoin_db.get_block_count();
    println!("{block_count} blocks found.");

    iter_blocks(&bitcoin_db, block_count)?;

    //
    //     Daemon::start();
    //     Daemon::wait_for_new_block(block_count - 1)?;
    // }

    Ok(())
}
