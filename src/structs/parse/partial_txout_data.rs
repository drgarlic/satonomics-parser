use super::RawAddress;

pub struct PartialTxoutData {
    pub sats: u64,
    pub raw_address: Option<RawAddress>,
    pub address_index_opt: Option<u32>,
}

impl PartialTxoutData {
    pub fn new(raw_address: Option<RawAddress>, sats: u64, address_index_opt: Option<u32>) -> Self {
        Self {
            raw_address,
            sats,
            address_index_opt,
        }
    }
}
