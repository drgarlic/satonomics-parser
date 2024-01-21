use std::{fs, io};

pub trait DatabaseGroup {
    fn export(&mut self) -> color_eyre::Result<()>;

    fn folder<'a>() -> &'a str;

    fn clear(&self) -> color_eyre::Result<(), io::Error> {
        fs::remove_dir_all(Self::folder())
    }
}
