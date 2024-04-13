use sanakirja::{direct_repr, Storable, UnsizedStorable};

use super::{AddressData, AddressType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct EmptyAddressData {
    pub address_type: AddressType,
    pub transfered: u64,
}
direct_repr!(EmptyAddressData);

impl EmptyAddressData {
    pub fn from_non_empty(non_empty: &AddressData) -> Self {
        if non_empty.sent != non_empty.received {
            dbg!(&non_empty);
            panic!("Trying to convert not empty wallet to empty !");
        }

        Self {
            address_type: non_empty.address_type,
            transfered: non_empty.sent,
        }
    }

    pub fn copy(&mut self, empty_address_data: &EmptyAddressData) {
        self.address_type = empty_address_data.address_type;
        self.transfered = empty_address_data.transfered;
    }
}
