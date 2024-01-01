use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::traits::Snapshot;

#[derive(Encode, Decode, Default, Deref, DerefMut)]
pub struct TxCounter(u32);

impl TxCounter {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl Snapshot for TxCounter {
    fn name<'a>() -> &'a str {
        "tx_counter"
    }
}
