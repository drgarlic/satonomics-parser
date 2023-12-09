use std::ops::{Deref, DerefMut};

use nohash_hasher::IntMap;

use crate::utils::{export_snapshot, import_snapshot_map};

pub struct TxoutIndexToValue(IntMap<usize, f64>);

const SNAPSHOT_NAME: &str = "height_to_aged__txout_index_to_value";

impl TxoutIndexToValue {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(
            import_snapshot_map::<f64>(SNAPSHOT_NAME, true)?
                .into_iter()
                .map(|(txout_index, value)| (txout_index.parse::<usize>().unwrap(), value))
                .collect::<IntMap<_, _>>(),
        ))
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        export_snapshot(SNAPSHOT_NAME, &self.0, false)
    }
}

impl Deref for TxoutIndexToValue {
    type Target = IntMap<usize, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxoutIndexToValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
