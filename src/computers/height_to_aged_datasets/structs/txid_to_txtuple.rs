use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use bitcoin_explorer::Txid;
use serde::{Deserialize, Serialize};

use crate::{
    structs::TxidMap,
    utils::{export_snapshot, import_snapshot_map},
};

#[derive(Debug)]
pub struct Txtuple {
    pub txid_index: usize,
    pub outputs_len: u32,
}

#[derive(Serialize, Deserialize)]
struct SerializedTxTuple(usize, u32);

impl Txtuple {
    pub fn new(txid_index: usize, outputs_len: u32) -> Self {
        Self {
            txid_index,
            outputs_len,
        }
    }

    fn import(serialized: &SerializedTxTuple) -> Self {
        Self {
            txid_index: serialized.0,
            outputs_len: serialized.1,
        }
    }

    fn export(&self) -> SerializedTxTuple {
        SerializedTxTuple(self.txid_index, self.outputs_len)
    }
}

pub struct TxidToTxtuple(TxidMap<Txtuple>);

const SNAPSHOT_NAME: &str = "height_to_aged__txid_to_txtuple";

impl TxidToTxtuple {
    pub fn import() -> color_eyre::Result<Self> {
        let child = TxidMap::new(None);

        child.borrow_mut_map().extend(
            import_snapshot_map::<SerializedTxTuple>(SNAPSHOT_NAME, true)?
                .iter()
                .map(|(txid, serialized)| {
                    (Txid::from_str(txid).unwrap(), Txtuple::import(serialized))
                }),
        );

        Ok(Self(child))
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        export_snapshot(
            SNAPSHOT_NAME,
            &self
                .borrow_map()
                .iter()
                .map(|(txid, tuple)| (txid.to_owned(), tuple.export()))
                .collect::<BTreeMap<_, _>>(),
            false,
        )
    }
}

impl Deref for TxidToTxtuple {
    type Target = TxidMap<Txtuple>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxidToTxtuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
