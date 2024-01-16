use std::thread;

use chrono::Local;

use crate::{
    bitcoin::check_if_height_safe,
    computers::{Databases, State},
};

use super::{
    structs::{
        AddressIndexToAddressData, AddressIndexToEmptyAddressData, Datasets, DateDataVec,
        RawAddressToAddressIndex, TxIndexToTxData, TxoutIndexToTxoutData,
    },
    Counters, TxidToTxIndex,
};

pub struct ExportedData<'a> {
    pub address_index_to_address_data: &'a AddressIndexToAddressData,
    pub address_index_to_empty_address_data: &'a mut AddressIndexToEmptyAddressData,
    pub block_count: usize,
    pub counters: &'a Counters,
    pub datasets: &'a Datasets,
    pub date_data_vec: &'a DateDataVec,
    pub height: usize,
    pub raw_address_to_address_index: &'a mut RawAddressToAddressIndex,
    pub tx_index_to_tx_data: &'a TxIndexToTxData,
    pub txid_to_tx_index: &'a mut TxidToTxIndex,
    pub txout_index_to_txout_data: &'a TxoutIndexToTxoutData,
}

pub fn export_all(
    ExportedData {
        address_index_to_address_data,
        address_index_to_empty_address_data,
        block_count,
        counters,
        datasets,
        date_data_vec,
        height,
        raw_address_to_address_index,
        tx_index_to_tx_data,
        txid_to_tx_index,
        txout_index_to_txout_data,
    }: ExportedData,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving... (Don't close !!)", Local::now());

    if check_if_height_safe(height, block_count) {
        thread::scope(|s| {
            s.spawn(|| address_index_to_address_data.export());
            s.spawn(|| address_index_to_empty_address_data.drain_export());
            s.spawn(|| counters.export());
            s.spawn(|| date_data_vec.export());
            s.spawn(|| raw_address_to_address_index.drain_export());
            s.spawn(|| tx_index_to_tx_data.export());
            s.spawn(|| txid_to_tx_index.drain_export());
            s.spawn(|| txout_index_to_txout_data.export());
        });
    }

    // datasets.export_if_needed(Some(height));

    Ok(())
}
