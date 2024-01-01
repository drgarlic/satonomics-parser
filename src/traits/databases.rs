use std::{fs, io};

use sanakirja::Error;

pub trait Databases
where
    Self: Sized,
{
    fn open(height: usize) -> color_eyre::Result<Self>;

    fn export(self) -> color_eyre::Result<(), Error>;

    fn clear() -> io::Result<()> {
        fs::remove_dir_all(Self::folder())
    }

    fn folder<'a>() -> &'a str;
}
