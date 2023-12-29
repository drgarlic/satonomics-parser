use serde::{Deserialize, Serialize};

use super::AddressData;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EmptyAddressData {
    pub sent: f64,
    pub received: f64,
}

impl EmptyAddressData {
    pub fn from_non_empty(non_empty: AddressData) -> Self {
        Self {
            sent: non_empty.sent,
            received: non_empty.received,
        }
    }
}
