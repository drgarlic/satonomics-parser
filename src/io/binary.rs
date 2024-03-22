use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use bincode::{
    config::{self},
    decode_from_std_read, encode_into_std_write, Decode, Encode,
};

pub struct Binary;

// TODO: Try https://docs.rs/bitcode/0.6.0-beta.1/bitcode/index.html

impl Binary {
    pub fn import<T, P>(path: P) -> color_eyre::Result<T>
    where
        T: Decode + Debug,
        P: AsRef<Path>,
    {
        let config = config::standard();

        let file = File::open(path)?;

        let mut reader = BufReader::new(file);

        let decoded = decode_from_std_read(&mut reader, config)?;

        Ok(decoded)
    }

    pub fn export<T, P>(path: P, value: &T) -> color_eyre::Result<()>
    where
        T: Encode,
        P: AsRef<Path>,
    {
        let config = config::standard();

        let file = File::create(path)?;

        let mut writer = BufWriter::new(file);

        encode_into_std_write(value, &mut writer, config)?;

        Ok(())
    }
}
