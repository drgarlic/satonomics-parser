use savefile::{load_file, save_file, Deserialize, Serialize};

pub struct Binary;

// NOTES:
// bincode 2.0: it was very consistent in terms of timing until it wasn't at around ~800 000 blocks processed with times between 100s and 3000s, might want to try again later
// savefile: less consistent maybe even slower but good enough for now (as of height ~350 000)
// rkyv: need to try but having an archived mirror of all serialized struct seems annoying

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
