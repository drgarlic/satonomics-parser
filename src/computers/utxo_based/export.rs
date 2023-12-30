use std::thread;

use chrono::Local;

use crate::traits::Snapshot;

use super::structs::{
    AddressCounter, AddressIndexToAddressData, AddressIndexToEmptyAddressData,
    AddressToAddressIndex, DateDataVec, TxIndexToTxData, TxidToTxIndex, TxoutIndexToTxoutData,
    UtxoDatasets,
};

pub struct ExportData<'a> {
    pub address_counter: &'a AddressCounter,
    pub address_index_to_address_data: &'a AddressIndexToAddressData,
    pub address_index_to_empty_address_data: AddressIndexToEmptyAddressData,
    pub address_to_address_index: AddressToAddressIndex,
    pub datasets: &'a UtxoDatasets,
    pub date_data_vec: &'a DateDataVec,
    pub height: usize,
    pub tx_index_to_tx_data: &'a TxIndexToTxData,
    pub txid_to_tx_index: &'a TxidToTxIndex,
    pub txout_index_to_txout_data: &'a TxoutIndexToTxoutData,
}

pub fn export_all(
    #[allow(unused_variables)] ExportData {
        address_counter,
        address_index_to_address_data,
        address_index_to_empty_address_data,
        address_to_address_index,
        datasets,
        date_data_vec,
        height,
        tx_index_to_tx_data,
        txid_to_tx_index,
        txout_index_to_txout_data,
    }: ExportData,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving... (Don't close !!)", Local::now());

    thread::scope(|s| {
        // s.spawn(|| datasets.export_if_needed(Some(height)));
        s.spawn(|| address_counter.export());
        s.spawn(|| address_index_to_address_data.export());
        s.spawn(|| address_index_to_empty_address_data.export());
        s.spawn(|| address_to_address_index.export());
        s.spawn(|| date_data_vec.export());
        s.spawn(|| tx_index_to_tx_data.export());
        s.spawn(|| txid_to_tx_index.export());
        s.spawn(|| txout_index_to_txout_data.export());
    });

    Ok(())
}
