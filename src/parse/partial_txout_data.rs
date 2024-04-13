use super::Address;

pub struct PartialTxoutData {
    pub sats: u64,
    pub address: Option<Address>,
    pub address_index_opt: Option<u32>,
}

impl PartialTxoutData {
    pub fn new(address: Option<Address>, sats: u64, address_index_opt: Option<u32>) -> Self {
        Self {
            address,
            sats,
            address_index_opt,
        }
    }
}
