use bincode::{config, Decode, Encode};
use redb::{RedbValue, TypeName};

use super::AddressData;

#[derive(Encode, Decode, Debug)]
pub struct EmptyAddressData {
    index: u32,
    data: AddressData,
}

impl RedbValue for EmptyAddressData {
    type SelfType<'a> = Self;
    type AsBytes<'a> = Vec<u8> where Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self
    where
        Self: 'a,
    {
        let config = config::standard();

        bincode::borrow_decode_from_slice(data, config).unwrap().0
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        let config = config::standard();

        bincode::encode_to_vec(value, config).unwrap()
    }

    fn type_name() -> TypeName {
        TypeName::new(stringify!(EmptyAddress))
    }
}
