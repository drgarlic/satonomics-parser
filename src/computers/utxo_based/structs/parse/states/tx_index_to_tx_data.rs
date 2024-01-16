use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::computers::TxData;

use super::State;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct TxIndexToTxData(BTreeMap<u32, TxData>);

impl State for TxIndexToTxData {
    fn name<'a>() -> &'a str {
        "tx_index_to_tx_data"
    }
}
