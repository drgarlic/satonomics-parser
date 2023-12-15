use std::thread;

use chrono::{Local, NaiveDate};

mod dataset_aged;
mod dataset_coinblocks;
mod dataset_coindays;
mod dataset_rewards;
mod datasets;
mod insert_data;

use dataset_aged::*;
use dataset_coinblocks::*;
use dataset_coindays::*;
use dataset_rewards::*;
pub use datasets::*;
pub use insert_data::*;

use crate::structs::HeightDatasets;

use super::{DateDataVec, TxidIndexToBlockPath, TxidToTxData, TxoutIndexToTxoutValue};

pub fn export_all(
    date: &NaiveDate,
    height: usize,
    datasets: &UtxoDatasets,
    date_data_vec: &DateDataVec,
    txid_to_tx_data: &TxidToTxData,
    txout_index_to_txout_value: &TxoutIndexToTxoutValue,
    txid_index_to_block_path: &TxidIndexToBlockPath,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving {date}... (Don't close !!)", Local::now());

    thread::scope(|s| {
        s.spawn(|| datasets.export_if_needed(Some(height)));
        s.spawn(|| date_data_vec.export());
        s.spawn(|| txid_to_tx_data.export());
        s.spawn(|| txout_index_to_txout_value.export());
        s.spawn(|| txid_index_to_block_path.export());
    });

    Ok(())
}
