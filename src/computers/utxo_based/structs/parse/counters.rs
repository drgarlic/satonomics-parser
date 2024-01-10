use bincode::{Decode, Encode};

use crate::{structs::Counter, traits::Snapshot};

#[derive(Encode, Decode, Default, Debug)]
pub struct Counters {
    pub addresses: Counter,
    pub txs: Counter,
    pub unknown_addresses: Counter,
    pub empty_addresses: Counter,
}

impl Counters {
    pub fn reset(&mut self) {
        self.addresses.reset();
        self.txs.reset();
        self.unknown_addresses.reset();
        self.empty_addresses.reset();
    }
}

impl Snapshot for Counters {
    fn name<'a>() -> &'a str {
        "counters"
    }
}
