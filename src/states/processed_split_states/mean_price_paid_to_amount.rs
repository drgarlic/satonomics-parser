use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};

use crate::{
    bitcoin::sats_to_btc,
    datasets::{PricePaidState, UnrealizedState},
};

#[derive(Deref, DerefMut, Default, Debug)]
pub struct MeanPricePaidInCentsToAmount(BTreeMap<u64, u64>);

impl MeanPricePaidInCentsToAmount {
    pub fn increment(&mut self, mean_price_paid_in_cents: u64, amount: u64) {
        *self.entry(mean_price_paid_in_cents).or_default() += amount;
    }

    pub fn decrement(&mut self, mean_price_paid_in_cents: u64, amount: u64) {
        let delete = {
            let _amount = self.get_mut(&mean_price_paid_in_cents).unwrap_or_else(|| {
                dbg!(mean_price_paid_in_cents, amount);
                panic!();
            });

            *_amount -= amount;

            amount == 0
        };

        if delete {
            self.remove(&mean_price_paid_in_cents).unwrap();
        }
    }

    pub fn compute_price_paid_state(&self, total_supply: u64) -> PricePaidState {
        let mut price_paid_state = PricePaidState::default();

        self.iter()
            .for_each(|(mean_price_paid_in_cent, sat_amount)| {
                let mean_price_paid = (*mean_price_paid_in_cent as f32) / 100.0;
                let btc_amount = sats_to_btc(*sat_amount);
                price_paid_state.iterate(mean_price_paid, btc_amount, *sat_amount, total_supply);
            });

        price_paid_state
    }

    pub fn compute_unrealized_state(&self, ref_price: f32) -> UnrealizedState {
        let mut unrealized_state = UnrealizedState::default();

        self.iter()
            .for_each(|(mean_price_paid_in_cent, sat_amount)| {
                let mean_price_paid = (*mean_price_paid_in_cent as f32) / 100.0;
                let btc_amount = sats_to_btc(*sat_amount);
                unrealized_state.iterate(mean_price_paid, ref_price, *sat_amount, btc_amount);
            });

        unrealized_state
    }
}
