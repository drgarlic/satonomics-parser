mod bitcoin;
mod computers;
mod run;
mod structs;
mod traits;
mod utils;

use crate::utils::Node;

use run::run;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Node::start();

    // loop {
    //     Node::wait_sync()?;

    //     Node::stop();

    let block_count = run()?;

    //     Node::start();

    //     Node::wait_for_new_block(block_count - 1)?;
    // }

    Ok(())
}
