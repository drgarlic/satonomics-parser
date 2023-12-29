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
type Key = Bytes;
type Value = U32<NativeEndian>;
type DB = Database<Key, Value>;

pub struct EmptyAddressToAddressIndex(DB);

impl EmptyAddressToAddressIndex {
    pub fn open(env: &HeedEnv, writer: &mut RwTxn) -> Result<Self, Error> {
        let db = env
            .create_database(writer, Some("empty_address_to_address_index"))
            .unwrap();

        Ok(Self(db))
    }
}

impl Deref for EmptyAddressToAddressIndex {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EmptyAddressToAddressIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
