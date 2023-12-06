use std::{collections::HashMap, fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

use super::{export_json, import_json_map, import_json_vec};

const SNAPSHOT_FOLDER: &str = "./snapshots";

pub fn import_snapshot_vec<T>(name: &str, default: bool) -> color_eyre::Result<Vec<T>>
where
    T: DeserializeOwned + Serialize,
{
    import_json_vec(
        Path::new(&format!("{SNAPSHOT_FOLDER}/{name}.json")),
        default,
    )
}

pub fn import_snapshot_map<T>(name: &str, default: bool) -> color_eyre::Result<HashMap<String, T>>
where
    T: DeserializeOwned + Serialize,
{
    import_json_map(
        Path::new(&format!("{SNAPSHOT_FOLDER}/{name}.json")),
        default,
    )
}

pub fn export_snapshot<T>(name: &str, value: &T, pretty: bool) -> color_eyre::Result<()>
where
    T: DeserializeOwned + Serialize,
{
    let path = format!("{SNAPSHOT_FOLDER}/{name}.json");

    fs::copy(&path, format!("{SNAPSHOT_FOLDER}/{name}__backup.json"))?;

    export_json(Path::new(&path), value, pretty)
}
