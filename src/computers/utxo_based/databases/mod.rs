use std::{io, thread};

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

    pub fn clear(&self) -> color_eyre::Result<(), io::Error> {
        self.address_index_to_empty_address_data.clear()?;
        self.raw_address_to_address_index.clear()?;
        self.txid_to_tx_index.clear()?;

        Ok(())
    }
}
