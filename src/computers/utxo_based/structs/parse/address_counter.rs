use std::ops::{Deref, DerefMut};

use bincode::{Decode, Encode};

use crate::traits::Snapshot;

#[derive(Encode, Decode, Default)]
pub struct AddressCounter(u32);

impl Snapshot for AddressCounter {
    fn name<'a>() -> &'a str {
        "address_counter"
    }
}

impl Deref for AddressCounter {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AddressCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
