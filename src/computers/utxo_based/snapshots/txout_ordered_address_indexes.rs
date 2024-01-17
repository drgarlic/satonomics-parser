use super::Snapshot;

#[derive(Default)]
pub struct TxOutOrderedAddressIndexes;

impl Snapshot for TxOutOrderedAddressIndexes {
    type Target = Vec<Option<u32>>;

    fn name<'a>() -> &'a str {
        "txout_ordered_address_indexes"
    }
}
