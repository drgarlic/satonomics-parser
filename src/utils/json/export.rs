use std::{fs, path::Path};

use bincode::{config, encode_to_vec, Encode};
use serde::Serialize;

pub fn export_pretty_json<T>(path: &Path, value: &T) -> color_eyre::Result<()>
where
    T: Serialize,
{
    let contents = serde_json::to_string_pretty(value)?;

    fs::write(path, contents)?;

    Ok(())
}

pub fn export_dirty_json<T>(path: &Path, value: &T) -> color_eyre::Result<()>
where
    T: Serialize,
{
    let contents = serde_json::to_string(value)?;

    fs::write(path, contents)?;

    Ok(())
}

pub fn export_binary<T>(path: &Path, value: &T) -> color_eyre::Result<()>
where
    T: Encode,
{
    let config = config::standard();

    let encoded = encode_to_vec(value, config)?;

    fs::write(path, encoded)?;

    Ok(())
}
