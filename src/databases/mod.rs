mod _trait;
mod address_index_to_empty_address_data;
mod metadata;
mod raw_address_to_address_index;
mod txid_to_tx_index;

use _trait::*;
pub use address_index_to_empty_address_data::*;
use metadata::*;
pub use raw_address_to_address_index::*;
pub use txid_to_tx_index::*;

pub struct Databases {
    pub address_index_to_empty_address_data: AddressIndexToEmptyAddressData,
    pub raw_address_to_address_index: RawAddressToAddressIndex,
    pub txid_to_tx_index: TxidToTxIndex,
}

impl Databases {
    pub fn import() -> Self {
        let address_index_to_empty_address_data = AddressIndexToEmptyAddressData::import();

        let raw_address_to_address_index = RawAddressToAddressIndex::import();

        let txid_to_tx_index = TxidToTxIndex::import();

        Self {
            address_index_to_empty_address_data,
            raw_address_to_address_index,
            txid_to_tx_index,
        }
    }

    pub fn export(&mut self) -> color_eyre::Result<()> {
        // Don't par them
        self.address_index_to_empty_address_data.export()?;
        self.raw_address_to_address_index.export()?;
        self.txid_to_tx_index.export()?;

        Ok(())
    }

    pub fn reset(&mut self, include_addresses: bool) {
        if include_addresses {
            let _ = self.address_index_to_empty_address_data.reset();
            let _ = self.raw_address_to_address_index.reset();
        }

        let _ = self.txid_to_tx_index.reset();
    }
}
