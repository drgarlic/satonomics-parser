use std::ops::{Deref, DerefMut};

use heed::{
    byteorder::NativeEndian,
    types::{Bytes, U32},
    Database, Error, RwTxn,
};

use super::HeedEnv;

/// Address is the string version of either:
/// - mono
/// - multi (sorted and joined)
// type Key = &'static [u8];
type Key = U32<NativeEndian>;
type Value = Bytes;
type DB = Database<Key, Value>;

pub struct AddressIndexToAddress(DB);

impl AddressIndexToAddress {
    pub fn open(env: &HeedEnv, writer: &mut RwTxn) -> Result<Self, Error> {
        let db = env
            .create_database(writer, Some("address_to_address_index"))
            .unwrap();

        Ok(Self(db))
    }
}

impl Deref for AddressIndexToAddress {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AddressIndexToAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
