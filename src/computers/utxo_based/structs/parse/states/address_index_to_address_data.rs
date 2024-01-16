use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::computers::AddressData;

use super::State;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct AddressIndexToAddressData(BTreeMap<u32, AddressData>);

impl State for AddressIndexToAddressData {
    fn name<'a>() -> &'a str {
        "address_index_to_address_data"
    }
}
