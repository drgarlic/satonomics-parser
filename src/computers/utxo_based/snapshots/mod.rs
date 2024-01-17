use std::io;

mod _trait;
mod empty_address_index_to_empty_address_data;
mod txin_ordered_tx_indexes;
mod txout_ordered_address_indexes;

pub use _trait::*;
pub use empty_address_index_to_empty_address_data::*;
pub use txin_ordered_tx_indexes::*;
pub use txout_ordered_address_indexes::*;

pub struct Snapshots {
    pub empty_address_index_to_empty_address_data: EmptyAddressIndexToEmptyAddressData,
    pub txin_ordered_tx_indexes: TxInOrderedTxIndexes,
    pub txout_ordered_address_indexes: TxOutOrderedAddressIndexes,
}

impl Snapshots {
    pub fn init() -> color_eyre::Result<Self, io::Error> {
        Ok(Self {
            empty_address_index_to_empty_address_data: EmptyAddressIndexToEmptyAddressData::init()?,
            txin_ordered_tx_indexes: TxInOrderedTxIndexes::init()?,
            txout_ordered_address_indexes: TxOutOrderedAddressIndexes::init()?,
        })
    }
}
