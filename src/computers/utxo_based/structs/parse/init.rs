use std::thread;

use chrono::Days;

use crate::{
    computers::utxo_based::structs::Datasets, structs::DateMap, traits::HeightDatasets,
    traits::Snapshot,
};

use super::{
    AddressCounter, AddressIndexToAddressData, DateDataVec, TxCounter, TxIndexToTxData,
    TxidToTxIndex, TxoutIndexToTxoutData, UnknownAddressCounter,
};

#[derive(Default)]
pub struct InitiatedParsers {
    pub address_counter: AddressCounter,
    pub address_index_to_address_data: AddressIndexToAddressData,
    pub date_data_vec: DateDataVec,
    pub height: usize,
    pub tx_counter: TxCounter,
    pub tx_index_to_tx_data: TxIndexToTxData,
    pub txid_to_tx_index: TxidToTxIndex,
    pub txout_index_to_txout_data: TxoutIndexToTxoutData,
    pub unknown_address_counter: UnknownAddressCounter,
}

impl InitiatedParsers {
    pub fn init(datasets: &Datasets, date_to_first_block: &DateMap<usize>) -> Self {
        Self::read(datasets, date_to_first_block).unwrap_or_default()
    }

    fn read(datasets: &Datasets, date_to_first_block: &DateMap<usize>) -> color_eyre::Result<Self> {
        let address_index_to_address_data_handle = thread::spawn(AddressIndexToAddressData::import);

        let tx_index_to_tx_data_handle = thread::spawn(TxIndexToTxData::import);

        let txout_index_to_txout_value_handle = thread::spawn(TxoutIndexToTxoutData::import);

        let txid_to_tx_index_handle = thread::spawn(TxidToTxIndex::import);

        let date_data_vec_handle = thread::spawn(DateDataVec::import);

        let mut address_counter = AddressCounter::import()?;
        let mut tx_counter = TxCounter::import()?;
        let mut unknown_address_counter = UnknownAddressCounter::import()?;

        let mut date_data_vec = date_data_vec_handle.join().unwrap()?;

        let max_date = date_data_vec.iter().map(|date_data| date_data.date).max();

        let mut txid_to_tx_index = txid_to_tx_index_handle.join().unwrap()?;

        let mut txout_index_to_txout_data = txout_index_to_txout_value_handle.join().unwrap()?;

        let mut tx_index_to_tx_data = tx_index_to_tx_data_handle.join().unwrap()?;

        let mut address_index_to_address_data =
            address_index_to_address_data_handle.join().unwrap()?;

        println!("max {max_date:#?}");

        let height = max_date
                .and_then(|date| date.checked_add_days(Days::new(1)))
                .and_then(|date| {
                    let min_last_height = datasets.get_min_last_height();

                    dbg!(min_last_height);

                    date_to_first_block.get(&date).map(|snapshot_start_height| {

                        if min_last_height.unwrap_or(0) < snapshot_start_height - 1 {
                            println!("snapshot_start_height {snapshot_start_height} > last_saved_height {min_last_height:?}");
                            println!("Starting over...");

                            address_index_to_address_data.clear();
                            date_data_vec.clear();
                            tx_index_to_tx_data.clear();
                            txid_to_tx_index.clear();
                            txout_index_to_txout_data.clear();

                            address_counter.reset();
                            tx_counter.reset();
                            unknown_address_counter.reset();

                            return 0;
                        }

                        snapshot_start_height
                    })
                }).unwrap_or(0);

        println!("h {height}");

        Ok(Self {
            address_counter,
            address_index_to_address_data,
            date_data_vec,
            height,
            tx_counter,
            tx_index_to_tx_data,
            txid_to_tx_index,
            txout_index_to_txout_data,
            unknown_address_counter,
        })
    }
}
