use chrono::{Local, NaiveDate};

mod dataset;
mod dataset_aged;
mod dataset_coinblocks;
mod dataset_rewards;
mod datasets;
mod insert_data;

use dataset::*;
use dataset_aged::*;
use dataset_rewards::*;
pub use datasets::*;
pub use insert_data::*;

use super::{DateDataVec, TxidIndexToBlockPath, TxidToTxData, TxoutIndexToTxoutValue};

pub fn export_all(
    date: &NaiveDate,
    height: usize,
    datasets: &Datasets,
    date_data_vec: &DateDataVec,
    txid_to_tx_data: &TxidToTxData,
    txout_index_to_txout_value: &TxoutIndexToTxoutValue,
    txid_index_to_block_path: &TxidIndexToBlockPath,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving {date}... (Don't close !!)", Local::now());

    datasets.export(Some(height))?;

    date_data_vec.export()?;

    txid_to_tx_data.export()?;

    txout_index_to_txout_value.export()?;

    txid_index_to_block_path.export()?;

    Ok(())
}
