use std::sync::Weak;

use super::{AddressData, WMutex};

#[derive(Default, Debug)]
pub struct AddressRealizedData {
    pub received: u64,
    pub sent: u64,
    pub profit: f32,
    pub loss: f32,
    /// Filled only after the block is parsed since it can come from different places
    pub address_data_opt: Option<Weak<WMutex<AddressData>>>,
    /// Filled only after the block is parsed since it depends on address_data_opt
    pub previous_amount_opt: Option<u64>,
    /// Filled only after the block is parsed since it depends on address_data_opt
    pub previous_mean_price_paid_opt: Option<f32>,
}
