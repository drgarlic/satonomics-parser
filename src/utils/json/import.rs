#![allow(dead_code)]
use std::{collections::HashMap, fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

pub fn import_json_vec<T>(path: &Path, default: bool) -> color_eyre::Result<Vec<T>>
where
    T: DeserializeOwned + Serialize,
{
    import_json::<Vec<T>>(path, if default { Some("[]") } else { None })
}

pub fn import_json_map<T>(path: &Path, default: bool) -> color_eyre::Result<HashMap<String, T>>
where
    T: DeserializeOwned + Serialize,
{
    import_json::<HashMap<String, T>>(path, if default { Some("{}") } else { None })
}

fn import_json<T>(path: &Path, default: Option<&str>) -> color_eyre::Result<T>
where
    T: DeserializeOwned + Serialize,
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
