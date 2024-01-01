use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::traits::Snapshot;

use super::TxData;

#[derive(Encode, Decode, Default, Deref, DerefMut)]
pub struct TxIndexToTxData(BTreeMap<u32, TxData>);

impl Snapshot for TxIndexToTxData {
    fn name<'a>() -> &'a str {
        "tx_index_to_tx_data"
    }
}
