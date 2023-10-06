use bitcoin_explorer::BitcoinDB;
use std::{fs, path::Path, time::Instant};

mod utils;

mod chunks;
mod sth;

fn main() -> color_eyre::Result<()> {
    println!();

    println!("Starting...");

    color_eyre::install()?;

    let timer = Instant::now();

    let path = Path::new("/Volumes/t7s/bitcoin");

    let db = BitcoinDB::new(path, true)?;

    let block_count = db.get_block_count();

    println!("{block_count} blocks found.");

    let outputs_folder_rawpath = "./outputs";
    fs::create_dir_all(outputs_folder_rawpath)?;

    let process_chunks = false;
    if process_chunks {
        chunks::process(&db, block_count, outputs_folder_rawpath)?;
    }

    sth::process(&db, block_count, outputs_folder_rawpath)?;

    println!("Done in {} seconds.", timer.elapsed().as_secs_f32());

    Ok(())
}
