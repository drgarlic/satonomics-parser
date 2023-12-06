use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

pub fn export_json<T>(path: &Path, value: &T, pretty: bool) -> color_eyre::Result<()>
where
    T: DeserializeOwned + Serialize,
{
    let contents = {
        if pretty {
            serde_json::to_string_pretty(value)?
        } else {
            serde_json::to_string(value)?
        }
    };

    fs::write(path, contents)?;

    Ok(())
}
