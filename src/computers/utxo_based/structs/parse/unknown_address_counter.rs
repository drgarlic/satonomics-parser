use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::{structs::Counter, traits::Snapshot};

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct UnknownAddressCounter(Counter);

impl Snapshot for UnknownAddressCounter {
    fn name<'a>() -> &'a str {
        "unknown_address_counter"
    }
}
