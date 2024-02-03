use crate::structs::RawAddressType;

#[derive(Debug)]
pub enum AddressFilter {
    FromTo { from: u64, to: u64 },
    AddressType(RawAddressType),
}

impl AddressFilter {
    pub fn new_from_to(from: u64, to: u64) -> Self {
        Self::FromTo { from, to }
    }

    #[inline(always)]
    pub fn check(&self, amount: &u64, address_type: &RawAddressType) -> bool {
        match self {
            Self::FromTo { from, to } => amount >= from && amount < to,
            Self::AddressType(_address_type) => address_type == _address_type,
        }
    }
}
