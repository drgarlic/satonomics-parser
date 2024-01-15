use std::{fs, io};

pub trait Databases
where
    Self: Sized,
{
    fn open(height: usize) -> color_eyre::Result<Self>;

    fn export(self) -> color_eyre::Result<()>;

    fn clear() -> io::Result<()> {
        fs::remove_dir_all(Self::folder())
    }

    fn folder<'a>() -> &'a str;
}
