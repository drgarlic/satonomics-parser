use super::{RawAddressSize, RawAddressType};

pub enum RawAddressSplit {
    Type(RawAddressType),
    Size(RawAddressSize),
}
