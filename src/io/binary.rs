use savefile::{load_file, save_file, Deserialize, Serialize};

pub struct Binary;

// TODO: Try https://docs.rs/bitcode/0.6.0-beta.1/bitcode/index.html

impl Binary {
    pub fn import<T>(path: &str) -> color_eyre::Result<T>
    where
        T: Deserialize,
    {
        Ok(load_file(path, 0)?)
    }

    pub fn export<T>(path: &str, value: &T) -> color_eyre::Result<()>
    where
        T: Serialize,
    {
        Ok(save_file(path, 0, value)?)
    }
}
