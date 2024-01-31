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

use crate::{
    bitcoin::BitcoinDB,
    iter_blocks::iter_blocks,
    structs::{Daemon, BITCOIN_DATADIR_RAW_PATH},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    loop {
        Daemon::stop();

        // Scoped to free bitcoin's lock
        let block_count = {
            let bitcoin_db = BitcoinDB::new(Path::new(BITCOIN_DATADIR_RAW_PATH), true)?;

            let block_count = bitcoin_db.get_block_count();
            println!("{block_count} blocks found.");

            iter_blocks(&bitcoin_db, block_count)?;

            block_count
        };

        Daemon::start();

        if Daemon::check_if_fully_synced()? {
            Daemon::wait_for_new_block(block_count - 1)?;
        } else {
            Daemon::wait_sync()?;
        }
    }

    // Ok(())
}
