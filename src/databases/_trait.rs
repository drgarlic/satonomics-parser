use std::{fs, io};

use crate::structs::databases_folder_path;

pub trait AnyDatabaseGroup
where
    Self: Sized,
{
    fn import() -> Self;

    fn export(&mut self) -> color_eyre::Result<()>;

    fn folder<'a>() -> &'a str;

    fn reset(&mut self) -> color_eyre::Result<(), io::Error> {
        println!("Reset {}", Self::folder());

        self.sub_reset();

        fs::remove_dir_all(Self::full_path())?;

        Ok(())
    }

    fn full_path() -> String {
        databases_folder_path(Self::folder())
    }

    fn sub_reset(&mut self);
}
