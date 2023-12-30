use ordered_float::OrderedFloat;
use sanakirja::{direct_repr, Storable, UnsizedStorable};

use super::AddressData;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EmptyAddressData {
    pub sent: OrderedFloat<f64>,
    pub received: OrderedFloat<f64>,
}

direct_repr!(EmptyAddressData);

impl EmptyAddressData {
    pub fn from_non_empty(non_empty: AddressData) -> Self {
        Self {
            sent: OrderedFloat(non_empty.sent),
            received: OrderedFloat(non_empty.received),
        }
    }
}
