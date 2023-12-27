use std::ops::{Deref, DerefMut};

use redb::*;

use super::DatabaseWriter;

/// Address is the string version of either:
/// - mono
/// - multi (sorted and joined)
type Key = &'static [u8];
type Value = u32;

pub struct AddressToAddressIndex<'db, 'writer>(Table<'db, 'writer, Key, Value>);

const TABLE_DEFINITION: TableDefinition<Key, Value> =
    TableDefinition::new("address_to_address_index");

impl<'db, 'writer> AddressToAddressIndex<'db, 'writer> {
    pub fn open(
        writer: &'writer DatabaseWriter<'db>,
    ) -> Result<AddressToAddressIndex<'db, 'writer>, Error> {
        Ok(AddressToAddressIndex(writer.open_table(TABLE_DEFINITION)?))
    }
}

impl<'db, 'writer> Deref for AddressToAddressIndex<'db, 'writer> {
    type Target = Table<'db, 'writer, Key, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'db, 'writer> DerefMut for AddressToAddressIndex<'db, 'writer> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
