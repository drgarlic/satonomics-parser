use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

pub fn export_json<T>(path: &Path, value: &T) -> color_eyre::Result<()>
where
    T: DeserializeOwned + Serialize,
{
    fs::write(path, serde_json::to_string_pretty(value)?)?;

    Ok(())
}
