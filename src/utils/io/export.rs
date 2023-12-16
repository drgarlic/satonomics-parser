use std::{fs, path::Path};

use bincode::{config, encode_to_vec, Encode};

pub fn export_json<T>(path: &Path, value: &T) -> color_eyre::Result<()>
where
    T: serde::Serialize,
{
    let contents = serde_json::to_string_pretty(value)?;

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
