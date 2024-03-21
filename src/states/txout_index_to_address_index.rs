use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::parse::TxoutIndex;

use super::AnyState;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct TxoutIndexToAddressIndex(BTreeMap<TxoutIndex, u32>);

impl AnyState for TxoutIndexToAddressIndex {
    fn name<'a>() -> &'a str {
        "txout_index_to_address_index"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
