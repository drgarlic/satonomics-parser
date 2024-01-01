use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::traits::Snapshot;

use super::AddressData;

#[derive(Encode, Decode, Default, Deref, DerefMut)]
pub struct AddressIndexToAddressData(BTreeMap<u32, AddressData>);

impl Snapshot for AddressIndexToAddressData {
    fn name<'a>() -> &'a str {
        "address_index_to_address_data"
    }
}
