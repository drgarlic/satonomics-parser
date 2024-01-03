use std::collections::BTreeMap;

use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::traits::Snapshot;

use super::{TxoutData, TxoutIndex};

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct TxoutIndexToTxoutData(BTreeMap<TxoutIndex, TxoutData>);

impl Snapshot for TxoutIndexToTxoutData {
    fn name<'a>() -> &'a str {
        "txout_index_to_txout_data"
    }
}
