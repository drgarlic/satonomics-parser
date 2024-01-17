use std::collections::BTreeMap;

use crate::computers::utxo_based::EmptyAddressData;

use super::Snapshot;

#[derive(Default)]
pub struct EmptyAddressIndexToEmptyAddressData;

impl Snapshot for EmptyAddressIndexToEmptyAddressData {
    type Target = BTreeMap<u32, EmptyAddressData>;

    fn name<'a>() -> &'a str {
        "empty_address_index_to_empty_address_data"
    }
}
