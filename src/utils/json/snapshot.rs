use std::path::Path;

use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use super::{export_binary, export_dirty_json, import_binary, import_json_vec};

const SNAPSHOT_FOLDER: &str = "./snapshots";

pub fn import_snapshot_vec<T>(name: &str, default: bool) -> color_eyre::Result<Vec<T>>
where
    T: DeserializeOwned,
{
    import_json_vec(
        Path::new(&format!("{SNAPSHOT_FOLDER}/{name}.json")),
        default,
    )
}

pub fn import_snapshot_map<T>(name: &str) -> color_eyre::Result<T>
where
    T: Decode,
{
    import_binary(Path::new(&format!("{SNAPSHOT_FOLDER}/{name}.bin")))
}

pub fn export_snapshot_json<T>(name: &str, value: &T) -> color_eyre::Result<()>
where
    T: Serialize,
{
    let path = format!("{SNAPSHOT_FOLDER}/{name}.json");

    export_dirty_json(Path::new(&path), value)
}

pub fn export_snapshot_bin<T>(name: &str, value: &T) -> color_eyre::Result<()>
where
    T: Encode,
{
    let path = format!("{SNAPSHOT_FOLDER}/{name}.bin");

    export_binary(Path::new(&path), value)
}
