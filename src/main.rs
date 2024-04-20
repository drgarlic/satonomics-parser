use std::path::Path;

use parser::{iter_blocks, BitcoinDB, BitcoinDaemon};

const BITCOIN_DATADIR_RAW_PATH: &str = "/Users/k/Developer/bitcoin";

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let deamon = BitcoinDaemon::new(BITCOIN_DATADIR_RAW_PATH);

    loop {
        deamon.stop();

        // Scoped to free bitcoin's lock
        let block_count = {
            let bitcoin_db = BitcoinDB::new(Path::new(BITCOIN_DATADIR_RAW_PATH), true)?;

            let block_count = bitcoin_db.get_block_count();
            println!("{block_count} blocks found.");

            iter_blocks(&bitcoin_db, block_count)?;

            block_count
        };

        deamon.start();

        if deamon.check_if_fully_synced()? {
            deamon.wait_for_new_block(block_count - 1)?;
        } else {
            deamon.wait_sync()?;
        }
    }

    // Ok(())
}

// let vec = Json::import::<Vec<f32>>("./price/close/height.json")?;

// vec.chunks(HEIGHT_MAP_CHUNK_SIZE)
//     .enumerate()
//     .for_each(|(index, chunk)| {
//         let _ = Json::export(
//             &format!(
//                 "./price/close/height/{}..{}.json",
//                 index * HEIGHT_MAP_CHUNK_SIZE,
//                 (index + 1) * HEIGHT_MAP_CHUNK_SIZE
//             ),
//             &SerializedHeightMap {
//                 version: 1,
//                 map: chunk.to_vec(),
//             },
//         );
//     });

// panic!();
