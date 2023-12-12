use std::collections::BTreeMap;

use bitcoin_explorer::Txid;

pub type TxidMap<T> = BTreeMap<Txid, T>;
