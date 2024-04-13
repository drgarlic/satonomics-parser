use super::{AddressSize, AddressType};

pub enum AddressSplit {
    All,
    Type(AddressType),
    Size(AddressSize),
}
