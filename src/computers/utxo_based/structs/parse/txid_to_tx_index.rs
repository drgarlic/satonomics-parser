use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::{structs::WTxid, traits::Snapshot};

#[derive(Encode, Decode, Default, Deref, DerefMut)]
pub struct TxidToTxIndex(BTreeMap<WTxid, u32>);

impl Snapshot for TxidToTxIndex {
    fn name<'a>() -> &'a str {
        "txid_to_tx_index"
    }
}
