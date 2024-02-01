use super::AddressData;

#[derive(Default, Debug)]
pub struct AddressRealizedData<'a> {
    pub received: u64,
    pub sent: u64,
    pub profit: f32,
    pub loss: f32,
    /// Filled only after the block is parsed since it can come from different places
    pub address_data_opt: Option<&'a AddressData>,
    /// Filled only after the block is parsed since it depends on address_data_opt
    pub previous_amount_opt: Option<u64>,
}
