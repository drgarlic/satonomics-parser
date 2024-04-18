mod _trait;
mod address_index_to_empty_address_data;
mod address_to_address_index;
mod metadata;
mod txid_to_tx_index;

use std::thread;

use _trait::*;
pub use address_index_to_empty_address_data::*;
pub use address_to_address_index::*;
use metadata::*;
pub use txid_to_tx_index::*;

use crate::utils::time;

pub struct Databases {
    pub address_index_to_empty_address_data: AddressIndexToEmptyAddressData,
    pub address_to_address_index: AddressToAddressIndex,
    pub txid_to_tx_index: TxidToTxIndex,
}

impl Databases {
    pub fn import() -> Self {
        let address_index_to_empty_address_data = AddressIndexToEmptyAddressData::import();

        let address_to_address_index = AddressToAddressIndex::import();

        let txid_to_tx_index = TxidToTxIndex::import();

        Self {
            address_index_to_empty_address_data,
            address_to_address_index,
            txid_to_tx_index,
        }
    }

    pub fn export(&mut self) -> color_eyre::Result<()> {
        thread::scope(|s| {
            s.spawn(|| {
                time("  Database address_index_to_empty_address_data", || {
                    self.address_index_to_empty_address_data.export()
                })
            });
            s.spawn(|| {
                time("  Database address_to_address_index", || {
                    self.address_to_address_index.export()
                })
            });
            s.spawn(|| {
                time("  Database txid_to_tx_index", || {
                    self.txid_to_tx_index.export()
                })
            });
        });

        Ok(())
    }

    pub fn reset(&mut self, include_addresses: bool) {
        if include_addresses {
            let _ = self.address_index_to_empty_address_data.reset();
            let _ = self.address_to_address_index.reset();
        }

        let _ = self.txid_to_tx_index.reset();
    }
}
