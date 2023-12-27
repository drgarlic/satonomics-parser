use bincode::{Decode, Encode};
use redb::RedbValue;

#[derive(Encode, Decode, Default, Debug)]
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
}

impl RedbValue for AddressData {
    type SelfType<'a> = Self;
    type AsBytes<'a> = Vec<u8> where Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self
    where
        Self: 'a,
    {
        let config = bincode::config::standard();

        bincode::borrow_decode_from_slice(data, config).unwrap().0
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        let config = bincode::config::standard();

        bincode::encode_to_vec(value, config).unwrap()
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new(stringify!(EmptyAddress))
    }
}
