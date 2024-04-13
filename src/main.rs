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

// let vec = Json::import::<Vec<f32>>("./price/close/height.json")?;

// let chunk_size = BLOCKS_PER_HAVLING_EPOCH / 8;

// vec.chunks(chunk_size)
//     .enumerate()
//     .for_each(|(index, chunk)| {
//         let _ = Json::export(
//             &format!(
//                 "./price/close/height/{}..{}.json",
//                 index * chunk_size,
//                 (index + 1) * chunk_size
//             ),
//             &chunk,
//         );
//     });
