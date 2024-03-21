use super::AddressData;

#[derive(Debug)]
pub struct AddressRealizedData {
    pub received: u64,
    pub sent: u64,
    pub profit: f32,
    pub loss: f32,
    /// Option
    pub initial_address_data: AddressData,
}

impl AddressRealizedData {
    pub fn default(initial_address_data: &AddressData) -> Self {
        Self {
            received: 0,
            sent: 0,
            profit: 0.0,
            loss: 0.0,
            initial_address_data: *initial_address_data,
        }
    }
}
