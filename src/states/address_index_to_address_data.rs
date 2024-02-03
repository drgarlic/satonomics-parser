use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::structs::AddressData;

use super::AnyState;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct AddressIndexToAddressData(BTreeMap<u32, AddressData>);

#[derive(Default, Deref, DerefMut, Debug)]
pub struct AddressIndexToAddressDataRef<'a>(BTreeMap<u32, &'a AddressData>);

// #[derive(Default)]
// struct SplitAddressIndexToAddressDataRef<'a> {
//     plankton: AddressIndexToAddressDataRef<'a>,
//     shrimp: AddressIndexToAddressDataRef<'a>,
//     crab: AddressIndexToAddressDataRef<'a>,
//     fish: AddressIndexToAddressDataRef<'a>,
//     shark: AddressIndexToAddressDataRef<'a>,
//     whale: AddressIndexToAddressDataRef<'a>,
//     humpback: AddressIndexToAddressDataRef<'a>,
//     megalodon: AddressIndexToAddressDataRef<'a>,

//     p2pk: AddressIndexToAddressDataRef<'a>,
//     p2pkh: AddressIndexToAddressDataRef<'a>,
//     p2sh: AddressIndexToAddressDataRef<'a>,
//     p2wpkh: AddressIndexToAddressDataRef<'a>,
//     p2wsh: AddressIndexToAddressDataRef<'a>,
//     p2tr: AddressIndexToAddressDataRef<'a>,
// }

impl AnyState for AddressIndexToAddressData {
    fn name<'a>() -> &'a str {
        "address_index_to_address_data"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
