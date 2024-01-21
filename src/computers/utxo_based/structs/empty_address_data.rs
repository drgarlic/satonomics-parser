use bincode::{Decode, Encode};
use sanakirja::{direct_repr, Storable, UnsizedStorable};

use super::{AddressData, RawAddressType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Decode, Encode)]
pub struct EmptyAddressData {
    pub address_type: RawAddressType,
    pub transfered: u64,
}
direct_repr!(EmptyAddressData);

impl EmptyAddressData {
    pub fn from_non_empty(non_empty: AddressData) -> Self {
        if non_empty.sent != non_empty.received {
            dbg!(&non_empty);
            panic!("Trying to convert not empty wallet to empty !");
        }

        Self {
            address_type: non_empty.address_type,
            transfered: non_empty.sent,
        }
    }
}
