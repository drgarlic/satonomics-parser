use std::{collections::BTreeMap, sync::Arc};

use crate::structs::{AddressData, WMutex};

use super::AnyState;
use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct AddressIndexToAddressData(BTreeMap<u32, Arc<WMutex<AddressData>>>);

impl AnyState for AddressIndexToAddressData {
    fn name<'a>() -> &'a str {
        "address_index_to_address_data"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
