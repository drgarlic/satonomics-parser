use std::ops::{Deref, DerefMut};

use redb::*;

use super::{AddressData, DatabaseWriter};

type Key = u32;
type Value = AddressData;

pub struct AddressIndexToEmptyAddressData<'db, 'writer>(Table<'db, 'writer, Key, Value>);

const TABLE_DEFINITION: TableDefinition<Key, Value> =
    TableDefinition::new("address_index_to_empty_address_data");

impl<'db, 'writer> AddressIndexToEmptyAddressData<'db, 'writer> {
    pub fn open(
        writer: &'writer DatabaseWriter<'db>,
    ) -> Result<AddressIndexToEmptyAddressData<'db, 'writer>, Error> {
        Ok(AddressIndexToEmptyAddressData(
            writer.open_table(TABLE_DEFINITION)?,
        ))
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        self.drain::<Key>(..)?;

        Ok(())
    }
}

impl<'db, 'writer> Deref for AddressIndexToEmptyAddressData<'db, 'writer> {
    type Target = Table<'db, 'writer, Key, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'db, 'writer> DerefMut for AddressIndexToEmptyAddressData<'db, 'writer> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
