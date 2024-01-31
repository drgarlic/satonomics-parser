use std::thread;

mod _trait;
mod address_index_to_empty_address_data;
mod raw_address_to_address_index;
mod txid_to_tx_index;

use _trait::*;
use address_index_to_empty_address_data::*;
use raw_address_to_address_index::*;
use txid_to_tx_index::*;

#[derive(Default)]
pub struct Databases {
    pub address_index_to_empty_address_data: AddressIndexToEmptyAddressData,
    pub raw_address_to_address_index: RawAddressToAddressIndex,
    pub txid_to_tx_index: TxidToTxIndex,
}

impl Databases {
    pub fn export(&mut self) -> color_eyre::Result<()> {
        thread::scope(|s| {
            s.spawn(|| self.address_index_to_empty_address_data.export());
            s.spawn(|| self.raw_address_to_address_index.export());
            s.spawn(|| self.txid_to_tx_index.export());
        });

        Ok(())
    }

    pub fn reset(&self, include_addresses: bool) {
        if include_addresses {
            println!("Reset address databases");

            let _ = self.address_index_to_empty_address_data.reset();
            let _ = self.raw_address_to_address_index.reset();
        }

        let _ = self.txid_to_tx_index.reset();
    }
}
