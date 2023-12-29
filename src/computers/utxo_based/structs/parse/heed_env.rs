use std::{
    fs,
    ops::{Deref, DerefMut},
    path::Path,
};

use heed::{Env, EnvFlags, EnvOpenOptions};

use crate::traits::SNAPSHOT_FOLDER;

pub struct HeedEnv(Env);

const ONE_GB: usize = 1024 * 1024 * 1024;

impl HeedEnv {
    pub fn import() -> color_eyre::Result<Self> {
        let str = Self::path();

        let path = Path::new(&str);

        fs::create_dir_all(path)?;

        let mut env_builder = EnvOpenOptions::new();

        unsafe {
            env_builder.flags(
                EnvFlags::NO_SYNC
                    | EnvFlags::NO_META_SYNC
                    // | EnvFlags::WRITE_MAP
                    // | EnvFlags::MAP_ASYNC
                    | EnvFlags::NO_LOCK
                    | EnvFlags::NO_TLS
                    | EnvFlags::NO_READ_AHEAD,
            );
        }

        let env = env_builder.map_size(100 * ONE_GB).max_dbs(10).open(path)?;

        Ok(Self(env))
    }

    pub fn path() -> String {
        format!("{SNAPSHOT_FOLDER}/heed")
    }
}

impl Default for HeedEnv {
    fn default() -> Self {
        let _ = fs::remove_file(Self::path());

        HeedEnv::import().unwrap()
    }
}

impl Deref for HeedEnv {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HeedEnv {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
