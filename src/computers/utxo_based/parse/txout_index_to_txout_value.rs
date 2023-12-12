use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::utils::{export_snapshot, import_snapshot_map};

pub struct TxoutIndexToTxoutValue(BTreeMap<usize, f64>);

const SNAPSHOT_NAME: &str = "height_to_aged__txout_index_to_txout_value";

impl TxoutIndexToTxoutValue {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(
            import_snapshot_map::<f64>(SNAPSHOT_NAME, true)?
                .into_iter()
                .map(|(txout_index, value)| (txout_index.parse::<usize>().unwrap(), value))
                .collect::<BTreeMap<_, _>>(),
        ))
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        export_snapshot(SNAPSHOT_NAME, &self.0, false)
    }
}

impl Deref for TxoutIndexToTxoutValue {
    type Target = BTreeMap<usize, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxoutIndexToTxoutValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
