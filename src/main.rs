mod bitcoin;
mod computers;
mod run;
mod structs;
mod traits;
mod utils;

// use crate::utils::{start_node, stop_node, wait_for_new_block, wait_node_sync};

use run::run;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // start_node();

    // loop {
    //     wait_node_sync()?;

    //     stop_node();

    let block_count = run()?;

    //     start_node();

    //     wait_for_new_block(block_count - 1)?;
    // }

    Ok(())
}
