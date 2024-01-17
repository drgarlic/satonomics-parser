use std::{fmt::Debug, fs, io};

use bincode::{Decode, Encode};

use crate::utils::{Binary, OUTPUTS_FOLDER_PATH};

// https://github.com/djkoloski/rust_serialization_benchmark
pub trait State
where
    Self: Encode + Decode + Debug,
{
    fn name<'a>() -> &'a str;

    fn create_dir_all() -> color_eyre::Result<(), io::Error> {
        fs::create_dir_all(Self::folder_path())
    }

    fn folder_path() -> String {
        format!("{OUTPUTS_FOLDER_PATH}/states")
    }

    fn full_path() -> String {
        let name = Self::name();

        let folder_path = Self::folder_path();

        format!("{folder_path}/{name}.bin")
    }

    fn import() -> color_eyre::Result<Self> {
        Self::create_dir_all()?;

        Binary::import(Self::full_path())
    }

    fn export(&self) -> color_eyre::Result<()> {
        Binary::export(Self::full_path(), self)
    }
}
