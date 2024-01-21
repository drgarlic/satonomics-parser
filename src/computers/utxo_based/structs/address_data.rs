use bincode::{Decode, Encode};

use crate::bitcoin::sats_to_btc;

use super::{EmptyAddressData, RawAddressType};

#[derive(Encode, Decode, Debug)]
pub struct AddressData {
    pub address_type: RawAddressType,
    pub amount: u64,
    pub sent: u64,
    pub received: u64,
    pub mean_price_paid: f32,
    pub outputs_len: u32,
}

impl AddressData {
    pub fn new(address_type: RawAddressType) -> Self {
        Self {
            address_type,
            amount: 0,
            sent: 0,
            received: 0,
            mean_price_paid: 0.0,
            outputs_len: 0,
        }
    }
}

impl AddressData {
    pub fn receive(&mut self, value: u64, price: f32) {
        let price = price as f64;
        let previous_mean_price_paid = self.mean_price_paid as f64;

        let previous_amount = self.amount;
        let new_amount = previous_amount + value;

        let btc_value = sats_to_btc(value);
        let priced_btc_value = btc_value * price;

        let previous_btc_amount = sats_to_btc(previous_amount);
        let new_btc_amount = sats_to_btc(new_amount);

        self.mean_price_paid = ((previous_mean_price_paid * previous_btc_amount + priced_btc_value)
            / new_btc_amount) as f32;

        self.amount = new_amount;

        self.received += value;

        self.outputs_len += 1;
    }

    pub fn spend(&mut self, value: u64, price: f32) -> f32 {
        let price = price as f64;
        let previous_mean_price_paid = self.mean_price_paid as f64;

        let previous_amount = self.amount;
        let new_amount = previous_amount - value;

        let btc_value = sats_to_btc(value);
        let priced_btc_value = btc_value * price;

        let previous_btc_amount = sats_to_btc(previous_amount);
        let new_btc_amount = sats_to_btc(new_amount);

        self.mean_price_paid = (((previous_mean_price_paid * previous_btc_amount)
            - priced_btc_value)
            / new_btc_amount) as f32;

        self.amount = new_amount;

        self.sent += value;

        self.outputs_len -= 1;

        (priced_btc_value - (btc_value * previous_mean_price_paid)) as f32
    }

    pub fn is_empty(&self) -> bool {
        self.amount == 0
    }

    pub fn from_empty(empty: EmptyAddressData) -> Self {
        Self {
            address_type: empty.address_type,
            amount: 0,
            sent: empty.transfered,
            received: empty.transfered,
            mean_price_paid: 0.0,
            outputs_len: 0,
        }
    }
}
