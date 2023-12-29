use std::ops::{Deref, DerefMut};

use heed::{
    byteorder::NativeEndian,
    types::{SerdeBincode, U32},
    Database, Error, RwTxn,
};

use super::{EmptyAddressData, HeedEnv};

type Key = U32<NativeEndian>;
type Value = SerdeBincode<EmptyAddressData>;
type DB = Database<Key, Value>;

pub struct AddressIndexToEmptyAddressData(DB);

impl AddressIndexToEmptyAddressData {
    pub fn open(env: &HeedEnv, writer: &mut RwTxn) -> Result<Self, Error> {
        let db = env
            .create_database(writer, Some("address_index_to_empty_address_data"))
            .unwrap();

        Ok(Self(db))
    }
}

impl Deref for AddressIndexToEmptyAddressData {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AddressIndexToEmptyAddressData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
