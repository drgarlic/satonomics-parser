use std::collections::BTreeMap;

use super::WTxid;

pub type TxidMap<T> = BTreeMap<WTxid, T>;
