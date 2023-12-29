use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use super::empty_address_data::EmptyAddressData;

#[derive(Encode, Decode, Default, Debug, Serialize, Deserialize)]
pub struct AddressData {
    pub amount: f64,
    pub sent: f64,
    pub received: f64,
    pub mean_price_paid: f32,
}

impl AddressData {
    pub fn receive(&mut self, value: f64, price: f32) {
        let previous_amount = self.amount;

        let new_amount = previous_amount + value;

        self.mean_price_paid = ((self.mean_price_paid as f64 * previous_amount
            + value * price as f64)
            / new_amount) as f32;

        self.amount = new_amount;
        self.received += value;
    }

    pub fn spend(&mut self, value: f64, price: f32) {
        let previous_amount = self.amount;
        let previous_mean_price_paid = self.mean_price_paid as f64;

        let new_amount = previous_amount - value;

        self.mean_price_paid = (((previous_mean_price_paid * previous_amount)
            - (value * price as f64))
            / new_amount) as f32;

        self.amount = new_amount;

        self.sent += value;
    }

    pub fn is_empty(&self) -> bool {
        self.amount == 0.0
    }

    pub fn from_empty(empty: EmptyAddressData) -> Self {
        Self {
            amount: 0.0,
            sent: empty.sent,
            received: empty.received,
            mean_price_paid: 0.0,
        }
    }
}
