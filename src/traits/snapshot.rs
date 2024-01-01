use std::{fs, path::Path};

use bincode::{Decode, Encode};

use crate::utils::{export_binary, import_binary};

pub const SNAPSHOTS_FOLDER: &str = "./snapshots";

// https://github.com/djkoloski/rust_serialization_benchmark
pub trait Snapshot
where
    Self: Encode + Decode,
{
    fn name<'a>() -> &'a str;

    fn format_path_str() -> String {
        let name = Self::name();

        format!("{SNAPSHOTS_FOLDER}/{name}.bin")
    }

    fn import() -> color_eyre::Result<Self> {
        fs::create_dir_all(SNAPSHOTS_FOLDER)?;

        import_binary(Path::new(&Self::format_path_str()))
    }

    fn export(&self) -> color_eyre::Result<()> {
        export_binary(Path::new(&Self::format_path_str()), &self)
    }
}
