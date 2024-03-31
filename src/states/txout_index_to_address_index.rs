use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};
use savefile_derive::Savefile;

use crate::parse::TxoutIndex;

use super::AnyState;

#[derive(Default, Deref, DerefMut, Debug, Savefile)]
pub struct TxoutIndexToAddressIndex(BTreeMap<TxoutIndex, u32>);

impl AnyState for TxoutIndexToAddressIndex {
    fn name<'a>() -> &'a str {
        "txout_index_to_address_index"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
