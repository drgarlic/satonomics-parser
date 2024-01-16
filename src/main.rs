mod bitcoin;
mod computers;
mod run;
mod structs;
mod utils;

use run::run;
use utils::{Node, Terminal};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    Terminal::increase_open_files_limit();

    // Node::start();
    //
    // loop {
    //     Node::wait_sync()?;
    //
    //     Node::stop();
    //
    //
    let block_count = run()?;
    //
    //     Node::start();
    //     Node::wait_for_new_block(block_count - 1)?;
    // }

    Ok(())
}
