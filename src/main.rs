use std::path::Path;

mod actions;
mod bitcoin;
mod databases;
mod datasets;
mod io;
mod parse;
mod price;
mod states;
mod utils;

use crate::{
    actions::iter_blocks,
    bitcoin::{BitcoinDB, Daemon, BITCOIN_DATADIR_RAW_PATH},
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
