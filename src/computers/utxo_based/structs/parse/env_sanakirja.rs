use std::{fs, path::Path};

use sanakirja::Env;

use crate::traits::SNAPSHOT_FOLDER;

pub struct EnvSanakirja();

impl EnvSanakirja {
    pub fn import(name: &str) -> color_eyre::Result<Env> {
        let str = format!("{SNAPSHOT_FOLDER}/{name}");

        let path = Path::new(&str);

        fs::create_dir_all(path)?;

        let env = Env::new(path.join(Path::new("db")), 4096 * 1000, 1).unwrap();

        Ok(env)
    }

    pub fn default(name: &str) -> Env {
        let _ = fs::remove_dir_all(format!("{SNAPSHOT_FOLDER}/{name}"));

        EnvSanakirja::import(name).unwrap()
    }
}
