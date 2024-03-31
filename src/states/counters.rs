use savefile_derive::Savefile;

use crate::parse::Counter;

use super::AnyState;

#[derive(Default, Debug, Savefile)]
pub struct Counters {
    pub unknown_addresses: Counter,
    pub empty_addresses: Counter,
}

impl Counters {}

impl AnyState for Counters {
    fn name<'a>() -> &'a str {
        "counters"
    }

    fn clear(&mut self) {
        self.unknown_addresses.reset();
        self.empty_addresses.reset();
    }
}
