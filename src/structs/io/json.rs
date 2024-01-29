use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use serde::{de::DeserializeOwned, Serialize};

pub struct Json;

impl Json {
    pub fn import<T, P>(path: P) -> color_eyre::Result<T>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn export<T, P>(path: P, value: &T) -> color_eyre::Result<()>
    where
        T: Serialize,
        P: AsRef<Path> + Debug,
    {
        let file = File::create(&path).unwrap_or_else(|_| {
            dbg!(&path);
            panic!("No such file or directory")
        });

        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, value)?;

        Ok(())
    }
}
