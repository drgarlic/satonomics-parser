use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::computers::utxo_based::TxoutIndex;

use super::State;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct TxoutIndexToSats(BTreeMap<TxoutIndex, u64>);

impl State for TxoutIndexToSats {
    fn name<'a>() -> &'a str {
        "txout_index_to_sats"
    }
}
