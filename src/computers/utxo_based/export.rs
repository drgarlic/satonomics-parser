use std::thread;

use chrono::Local;

use crate::utils::Snapshot;

use super::structs::{
    AddressIndexToAddressData, DateDataVec, TxIndexToTxData, TxidToTxIndex, TxoutIndexToTxoutData,
    UtxoDatasets,
};

pub struct ExportData<'a> {
    pub height: usize,
    pub datasets: &'a UtxoDatasets,
    pub address_index_to_address_data: &'a AddressIndexToAddressData,
    pub date_data_vec: &'a DateDataVec,
    pub txid_to_tx_index: &'a TxidToTxIndex,
    pub txout_index_to_txout_data: &'a TxoutIndexToTxoutData,
    pub tx_index_to_tx_data: &'a TxIndexToTxData,
}

pub fn export_all(
    #[allow(unused_variables)] ExportData {
        height,
        datasets,
        address_index_to_address_data,
        date_data_vec,
        txid_to_tx_index,
        txout_index_to_txout_data,
        tx_index_to_tx_data,
    }: ExportData,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving... (Don't close !!)", Local::now());

    thread::scope(|s| {
        // s.spawn(|| datasets.export_if_needed(Some(height)));
        s.spawn(|| address_index_to_address_data.export());
        s.spawn(|| date_data_vec.export());
        s.spawn(|| txid_to_tx_index.export());
        s.spawn(|| txout_index_to_txout_data.export());
        s.spawn(|| tx_index_to_tx_data.export());
    });

    Ok(())
}
