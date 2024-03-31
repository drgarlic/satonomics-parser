use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};
use savefile_derive::Savefile;

use crate::parse::TxoutIndex;

use super::AnyState;

#[derive(Default, Deref, DerefMut, Debug, Savefile)]
pub struct TxoutIndexToSats(BTreeMap<TxoutIndex, u64>);

impl AnyState for TxoutIndexToSats {
    fn name<'a>() -> &'a str {
        "txout_index_to_sats"
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
