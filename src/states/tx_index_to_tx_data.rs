use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};
use savefile_derive::Savefile;

use crate::parse::TxData;

use super::AnyState;

#[derive(Default, Deref, DerefMut, Debug, Savefile)]
pub struct TxIndexToTxData(BTreeMap<u32, TxData>);

impl AnyState for TxIndexToTxData {
    fn name<'a>() -> &'a str {
        "tx_index_to_tx_data"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
