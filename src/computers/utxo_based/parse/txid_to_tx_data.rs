use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use bitcoin_explorer::Txid;

use crate::{
    structs::TxidMap,
    utils::{export_snapshot, import_snapshot_map},
};

use super::{SerializedTxData, TxData};

pub struct TxidToTxData(TxidMap<TxData>);

const SNAPSHOT_NAME: &str = "height_to_aged__txid_to_tx_data";

impl TxidToTxData {
    pub fn import() -> color_eyre::Result<Self> {
        let mut child = TxidMap::new();

        child.extend(
            import_snapshot_map::<SerializedTxData>(SNAPSHOT_NAME, true)?
                .iter()
                .map(|(txid, serialized)| {
                    (
                        Txid::from_str(txid).unwrap(),
                        TxData::deserialize(serialized),
                    )
                }),
        );

        Ok(Self(child))
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        export_snapshot(
            SNAPSHOT_NAME,
            &self
                .iter()
                .map(|(txid, tx_data)| (txid.to_owned(), tx_data.serialize()))
                .collect::<BTreeMap<_, _>>(),
            false,
        )
    }
}

impl Deref for TxidToTxData {
    type Target = TxidMap<TxData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxidToTxData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
