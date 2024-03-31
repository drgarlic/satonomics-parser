use savefile_derive::Savefile;
use std::{
    fmt::Debug,
    fs, io,
    ops::{Deref, DerefMut},
};

use crate::{
    io::Binary,
    parse::{Counter, WNaiveDate},
};

#[derive(Savefile, Default, Debug)]
pub struct Metadata {
    path: String,
    data: MetadataData,
}

impl Deref for Metadata {
    type Target = MetadataData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Metadata {
    pub fn import(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            data: MetadataData::import(path).unwrap_or_default(),
        }
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.data.export(&self.path)
    }

    pub fn reset(&mut self) {
        let _ = self.data.reset(&self.path);
    }
}

#[derive(Savefile, Default, Debug)]
pub struct MetadataData {
    pub len: Counter,
    pub last_block: Option<u64>,
    pub last_date: Option<WNaiveDate>,
}

impl MetadataData {
    fn name<'a>() -> &'a str {
        "metadata"
    }

    fn full_path(folder_path: &str) -> String {
        let name = Self::name();
        format!("{folder_path}/{name}.bin")
    }

    pub fn import(path: &str) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Binary::import(&Self::full_path(path))
    }

    pub fn export(&self, path: &str) -> color_eyre::Result<()> {
        Binary::export(&Self::full_path(path), self)
    }

    pub fn reset(&mut self, path: &str) -> color_eyre::Result<(), io::Error> {
        self.clear();

        fs::remove_file(Self::full_path(path))
    }

    fn clear(&mut self) {
        self.len.reset();
        self.last_block = None;
        self.last_date = None;
    }
}
