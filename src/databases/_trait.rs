use std::{fs, io};

use crate::structs::databases_folder_path;

pub trait AnyDatabaseGroup {
    fn export(&mut self) -> color_eyre::Result<()>;

    fn folder<'a>() -> &'a str;

    fn reset(&self) -> color_eyre::Result<(), io::Error> {
        fs::remove_dir_all(databases_folder_path(Self::folder()))
    }
}
