use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bincode::{Decode, Encode};

use crate::utils::Snapshot;

use super::AddressData;

#[derive(Encode, Decode, Default)]
pub struct AddressIndexToAddressData(BTreeMap<u32, AddressData>);

impl Snapshot for AddressIndexToAddressData {
    fn name<'a>() -> &'a str {
        "height_to_aged__address_index_to_address_data"
    }
}

impl Deref for AddressIndexToAddressData {
    type Target = BTreeMap<u32, AddressData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AddressIndexToAddressData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
