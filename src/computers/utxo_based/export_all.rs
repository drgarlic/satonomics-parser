use std::thread;

use chrono::Local;

use crate::{bitcoin::check_if_height_safe, computers::utxo_based::Databases};

use super::{Datasets, States};

pub struct ExportedData<'a> {
    pub block_count: usize,
    pub databases: &'a mut Databases,
    pub datasets: &'a Datasets,
    pub height: usize,
    pub states: &'a States,
}

pub fn export_all(
    ExportedData {
        block_count,
        databases,
        datasets,
        height,
        states,
    }: ExportedData,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving... (Don't close !!)", Local::now());

    if check_if_height_safe(height, block_count) {
        thread::scope(|s| {
            s.spawn(|| databases.drain_export());
            s.spawn(|| states.export());
        });
    }

    // datasets.export_if_needed(Some(height));

    Ok(())
}
