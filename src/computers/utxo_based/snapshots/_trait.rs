use std::{fmt::Debug, fs, io};

use bincode::{Decode, Encode};

use crate::utils::{Binary, OUTPUTS_FOLDER_PATH};

// https://github.com/djkoloski/rust_serialization_benchmark
pub trait Snapshot
where
    Self: Default,
    Self::Target: Encode + Decode + Debug,
{
    type Target;

    fn name<'a>() -> &'a str;

    fn init() -> color_eyre::Result<Self, io::Error> {
        Self::create_dir_all()?;

        Ok(Self::default())
    }

    fn create_dir_all() -> color_eyre::Result<(), io::Error> {
        fs::create_dir_all(Self::folder_path())
    }

    fn folder_path() -> String {
        let name = Self::name();

        format!("{OUTPUTS_FOLDER_PATH}/snapshots/{name}")
    }

    fn full_path(height: usize) -> String {
        let folder_path = Self::folder_path();

        format!("{folder_path}/{height}.bin")
    }

    fn import(&self, height: usize) -> color_eyre::Result<Self::Target> {
        Binary::import(Self::full_path(height))
    }

    fn export(&self, height: usize, value: &Self::Target) -> color_eyre::Result<()> {
        Binary::export(Self::full_path(height), value)
    }
}
