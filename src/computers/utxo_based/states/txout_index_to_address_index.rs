use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::computers::utxo_based::TxoutIndex;

use super::State;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct TxoutIndexToAddressIndex(BTreeMap<TxoutIndex, u32>);

impl State for TxoutIndexToAddressIndex {
    fn name<'a>() -> &'a str {
        "txout_index_to_address_index"
    }
}
