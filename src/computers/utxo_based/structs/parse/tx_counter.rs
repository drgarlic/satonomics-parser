use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::{structs::Counter, traits::Snapshot};

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct TxCounter(Counter);

impl Snapshot for TxCounter {
    fn name<'a>() -> &'a str {
        "tx_counter"
    }
}
