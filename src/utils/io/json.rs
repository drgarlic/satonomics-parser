use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use serde::de::DeserializeOwned;

pub struct Json;

impl Json {
    pub fn import_vec<T, P>(path: P) -> Vec<T>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        Self::import_json::<Vec<T>, P>(path).unwrap_or_default()
    }

    pub fn import_map<T, P>(path: P) -> HashMap<String, T>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        Self::import_json::<HashMap<String, T>, P>(path).unwrap_or_default()
    }

    fn import_json<T, P>(path: P) -> color_eyre::Result<T>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        Ok(serde_json::from_slice(&fs::read(path)?)?)
    }

    pub fn export<T, P>(path: &P, value: &T) -> color_eyre::Result<()>
    where
        T: serde::Serialize,
        P: AsRef<Path>,
    {
        let contents = serde_json::to_string_pretty(value)?;

        fs::write(path, contents)?;

        Ok(())
    }
}
