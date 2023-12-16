use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

use crate::utils::{import_json_array, import_json_map};

use super::export_json;

pub fn push_to_saved_vec<T>(path: &Path, value: T) -> color_eyre::Result<()>
where
    T: DeserializeOwned + Serialize,
{
    let mut list = import_json_array(path, true)?;

    list.push(value);

    export_json(path, &list)?;

    Ok(())
}

pub fn insert_to_saved_map<T>(path: &Path, key: String, value: T) -> color_eyre::Result<()>
where
    T: DeserializeOwned + Serialize,
{
    let mut map = import_json_map(path, true)?;

    map.insert(key, value);

    export_json(path, &map)?;

    Ok(())
}
