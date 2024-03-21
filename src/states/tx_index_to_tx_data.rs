use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::parse::TxData;

use super::AnyState;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct TxIndexToTxData(BTreeMap<u32, TxData>);

impl AnyState for TxIndexToTxData {
    fn name<'a>() -> &'a str {
        "tx_index_to_tx_data"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
