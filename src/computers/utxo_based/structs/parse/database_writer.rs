use std::ops::{Deref, DerefMut};

use redb::*;

use super::{AddressIndexToEmptyAddressData, Database};

// pub const ADDRESS_INDEX_TO_ADDRESS_TABLE: TableDefinition<u32, &[u8]> =
//     TableDefinition::new("address_index_to_address");

pub struct DatabaseWriter<'db>(WriteTransaction<'db>);

impl<'db> DatabaseWriter<'db> {
    pub fn begin(database: &'db Database) -> Result<Self, Error> {
        let writer = database.begin_write()?;

        Ok(Self(writer))
    }

    pub fn commit(self) -> Result<(), CommitError> {
        self.0.commit()
    }

    pub fn drain_all(self) -> Result<(), Error> {
        AddressIndexToEmptyAddressData::open(&self)?.clear()?;

        self.commit()?;

        Ok(())
    }
}

impl<'db> Deref for DatabaseWriter<'db> {
    type Target = WriteTransaction<'db>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'db> DerefMut for DatabaseWriter<'db> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
