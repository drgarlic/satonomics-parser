use bincode::{Decode, Encode};
use ordered_float::OrderedFloat;
use sanakirja::{direct_repr, Storable, UnsizedStorable};

use super::RawAddressType;

#[derive(Encode, Decode, Debug)]
pub struct AddressData {
    pub address_type: RawAddressType,
    pub amount: f64,
    pub sent: f64,
    pub received: f64,
    pub mean_price_paid: f32,
}

impl AddressData {
    pub fn new(address_type: RawAddressType) -> Self {
        Self {
            address_type,
            amount: 0.0,
            sent: 0.0,
            received: 0.0,
            mean_price_paid: 0.0,
        }
    }
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
            address_type: empty.address_type,
            amount: 0.0,
            sent: empty.sent.0,
            received: empty.received.0,
            mean_price_paid: 0.0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EmptyAddressData {
    pub address_type: RawAddressType,
    pub sent: OrderedFloat<f64>,
    pub received: OrderedFloat<f64>,
}
direct_repr!(EmptyAddressData);

impl EmptyAddressData {
    pub fn from_non_empty(non_empty: AddressData) -> Self {
        Self {
            address_type: non_empty.address_type,
            sent: OrderedFloat(non_empty.sent),
            received: OrderedFloat(non_empty.received),
        }
    }
}
