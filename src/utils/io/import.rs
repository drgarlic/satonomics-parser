#![allow(dead_code)]
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::Path,
};

use bincode::{config, decode_from_slice, Decode};
use serde::de::DeserializeOwned;

pub fn import_json_vec<T>(path: &Path, default: bool) -> color_eyre::Result<Vec<T>>
where
    T: DeserializeOwned,
{
    import_json::<Vec<T>>(path, if default { Some("[]") } else { None })
}

pub fn import_json_map<T>(path: &Path, default: bool) -> color_eyre::Result<HashMap<String, T>>
where
    T: DeserializeOwned,
{
    import_json::<HashMap<String, T>>(path, if default { Some("{}") } else { None })
}

fn import_json<T>(path: &Path, default: Option<&str>) -> color_eyre::Result<T>
where
    T: DeserializeOwned,
{
    let string = {
        if let Some(default) = default {
            fs::read_to_string(path).unwrap_or(default.to_owned())
        } else {
            fs::read_to_string(path)?
        }
    };

    Ok(serde_json::from_str(&string)?)
}

pub fn import_binary<T>(path: &Path) -> color_eyre::Result<T>
where
    T: Decode,
{
    let config = config::standard();

    let mut file = File::open(path)?;

    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    let decoded: T = decode_from_slice(&buffer, config)?.0;

    Ok(decoded)
}
