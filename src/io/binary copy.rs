use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, BufWriter},
};

use bincode::{
    config::{self},
    decode_from_std_read, encode_into_std_write, Decode, Encode,
};

pub struct Binary;

// TODO: Try https://docs.rs/bitcode/0.6.0-beta.1/bitcode/index.html

impl Binary {
    pub fn import<T>(path: &str) -> color_eyre::Result<T>
    where
        T: Decode + Debug,
    {
        let config = config::standard();

        let file = File::open(path)?;

        let mut reader = BufReader::new(file);

        let decoded = decode_from_std_read(&mut reader, config)?;

        Ok(decoded)
    }

    pub fn export<T>(path: &str, value: &T) -> color_eyre::Result<()>
    where
        T: Encode,
    {
        let config = config::standard();

        let file = File::create(path).unwrap_or_else(|_| {
            dbg!(&path);
            panic!("No such file or directory")
        });

        let mut writer = BufWriter::new(file);

        encode_into_std_write(value, &mut writer, config)?;

        Ok(())
    }
}
