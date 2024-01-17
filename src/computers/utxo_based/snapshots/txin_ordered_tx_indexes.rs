use super::Snapshot;

#[derive(Default)]
pub struct TxInOrderedTxIndexes;

impl Snapshot for TxInOrderedTxIndexes {
    type Target = Vec<Option<u32>>;

    fn name<'a>() -> &'a str {
        "txin_ordered_tx_indexes"
    }
}
