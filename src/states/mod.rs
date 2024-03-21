use std::thread;

mod _trait;
mod address_index_to_address_data;
mod counters;
mod date_data_vec;
mod processed_addresses_split_states;
mod tx_index_to_tx_data;
mod txout_index_to_address_index;

mod txout_index_to_sats;

pub use _trait::*;
use address_index_to_address_data::*;
use counters::*;
use date_data_vec::*;
pub use processed_addresses_split_states::*;
use tx_index_to_tx_data::*;
use txout_index_to_address_index::*;
use txout_index_to_sats::*;

#[derive(Default)]
pub struct States {
    // TODO: Change to Option
    pub address_index_to_address_data: AddressIndexToAddressData,
    pub counters: Counters,
    pub date_data_vec: DateDataVec,
    // TODO: Change to Option
    pub split_address: SplitVariousAddressStates,
    pub tx_index_to_tx_data: TxIndexToTxData,
    pub txout_index_to_address_index: TxoutIndexToAddressIndex,
    pub txout_index_to_sats: TxoutIndexToSats,
}

impl States {
    pub fn import(compute_address_states: bool) -> color_eyre::Result<Self> {
        let address_index_to_address_data_handle = thread::spawn(AddressIndexToAddressData::import);

        let tx_index_to_tx_data_handle = thread::spawn(TxIndexToTxData::import);

        let txout_index_to_sats_handle = thread::spawn(TxoutIndexToSats::import);

        let txout_index_to_address_index_handle = thread::spawn(TxoutIndexToAddressIndex::import);

        let date_data_vec_handle = thread::spawn(DateDataVec::import);

        let counters = Counters::import()?;

        let date_data_vec = date_data_vec_handle.join().unwrap()?;

        let txout_index_to_address_index = txout_index_to_address_index_handle.join().unwrap()?;

        let txout_index_to_sats = txout_index_to_sats_handle.join().unwrap()?;

        let tx_index_to_tx_data = tx_index_to_tx_data_handle.join().unwrap()?;

        // TODO: Use compute_address_states
        let address_index_to_address_data = address_index_to_address_data_handle.join().unwrap()?;

        // TODO: Use compute_address_states
        let split_address = SplitVariousAddressStates::init(&address_index_to_address_data);

        Ok(Self {
            address_index_to_address_data,
            counters,
            date_data_vec,
            split_address,
            tx_index_to_tx_data,
            txout_index_to_address_index,
            txout_index_to_sats,
        })
    }

    pub fn reset(&mut self) {
        println!("Reseting all states...");

        let _ = self.address_index_to_address_data.reset();
        let _ = self.counters.reset();
        let _ = self.date_data_vec.reset();
        let _ = self.tx_index_to_tx_data.reset();
        let _ = self.txout_index_to_address_index.reset();
        let _ = self.txout_index_to_sats.reset();

        self.split_address = SplitVariousAddressStates::default();
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        thread::scope(|s| {
            s.spawn(|| self.address_index_to_address_data.export().unwrap());
            s.spawn(|| self.counters.export().unwrap());
            s.spawn(|| self.date_data_vec.export().unwrap());
            s.spawn(|| self.tx_index_to_tx_data.export().unwrap());
            s.spawn(|| self.txout_index_to_address_index.export().unwrap());
            s.spawn(|| self.txout_index_to_sats.export().unwrap());
        });

        Ok(())
    }
}
