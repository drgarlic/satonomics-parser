use std::thread;

use chrono::Local;

use crate::traits::{Databases, Snapshot};

use super::{
    structs::{
        AddressIndexToAddressData, AddressIndexToEmptyAddressData, Datasets, DateDataVec,
        RawAddressToAddressIndex, TxIndexToTxData, TxoutIndexToTxoutData,
    },
    Counters, TxidToTxIndex,
};

pub struct ExportedData<'a> {
    pub address_index_to_address_data: &'a AddressIndexToAddressData,
    pub address_index_to_empty_address_data: AddressIndexToEmptyAddressData,
    pub address_to_address_index: RawAddressToAddressIndex,
    pub counters: &'a Counters,
    pub datasets: &'a Datasets,
    pub date_data_vec: &'a DateDataVec,
    pub height: usize,
    pub tx_index_to_tx_data: &'a TxIndexToTxData,
    pub txid_to_tx_index: TxidToTxIndex,
    pub txout_index_to_txout_data: &'a TxoutIndexToTxoutData,
}

pub fn export_all(
    ExportedData {
        address_index_to_address_data,
        address_index_to_empty_address_data,
        address_to_address_index,
        counters,
        datasets,
        date_data_vec,
        height,
        tx_index_to_tx_data,
        txid_to_tx_index,
        txout_index_to_txout_data,
    }: ExportedData,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving... (Don't close !!)", Local::now());

    thread::scope(|s| {
        s.spawn(|| address_index_to_address_data.export());
        s.spawn(|| address_index_to_empty_address_data.export());
        s.spawn(|| address_to_address_index.export());
        s.spawn(|| counters.export());
        s.spawn(|| date_data_vec.export());
        s.spawn(|| tx_index_to_tx_data.export());
        s.spawn(|| txid_to_tx_index.export());
        s.spawn(|| txout_index_to_txout_data.export());
    });

    // datasets.export_if_needed(Some(height));

    Ok(())
}
