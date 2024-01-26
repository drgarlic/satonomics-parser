use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::structs::TxoutIndex;

use super::AnyState;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct TxoutIndexToSats(BTreeMap<TxoutIndex, u64>);

impl AnyState for TxoutIndexToSats {
    fn name<'a>() -> &'a str {
        "txout_index_to_sats"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
