use std::{
    fmt::Debug,
    fs::{self, File},
    io::Read,
    path::Path,
};

use bincode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

pub struct Binary;

impl Binary {
    pub fn import<T, P>(path: P) -> color_eyre::Result<T>
    where
        T: Decode + Debug,
        P: AsRef<Path>,
    {
        let config = config::standard();

        let mut file = File::open(path)?;

        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer)?;

        let decoded = decode_from_slice(&buffer, config)?.0;

        Ok(decoded)
    }

    pub fn export<T, P>(path: P, value: &T) -> color_eyre::Result<()>
    where
        T: Encode,
        P: AsRef<Path>,
    {
        let config = config::standard();

        let encoded = encode_to_vec(value, config)?;

        fs::write(path, encoded)?;

        Ok(())
    }
}
