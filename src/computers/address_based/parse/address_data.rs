use serde::{Deserialize, Serialize};

pub struct AddressData {
    pub amount: u32,
    pub sent: f64,
    pub received: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedAddressData(u32, f64, f64);
