use super::{RawAddressSize, RawAddressType};

pub enum RawAddressSplit {
    All,
    Type(RawAddressType),
    Size(RawAddressSize),
}
