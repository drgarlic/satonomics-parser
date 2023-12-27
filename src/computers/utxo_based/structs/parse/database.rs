use std::{
    fs,
    ops::{Deref, DerefMut},
};

use redb::{Database as RedbDatabase, *};

use crate::utils::SNAPSHOT_FOLDER;

use super::DatabaseWriter;

pub struct Database(RedbDatabase);

const ONE_GB: usize = 1024 * 1024 * 1024;

impl Database {
    pub fn import() -> color_eyre::Result<Self> {
        let database = RedbDatabase::builder()
            .set_cache_size(10 * ONE_GB)
            .create(Self::path())?;

        Ok(Self(database))
    }

    pub fn path() -> String {
        format!("{SNAPSHOT_FOLDER}/database.redb")
    }

    pub fn clear(&self) -> Result<(), Error> {
        DatabaseWriter::begin(self)?.drain_all()
    }
}

impl Default for Database {
    fn default() -> Self {
        fs::remove_file(Self::path()).unwrap();

        Database::import().unwrap()
    }
}

impl Deref for Database {
    type Target = RedbDatabase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
