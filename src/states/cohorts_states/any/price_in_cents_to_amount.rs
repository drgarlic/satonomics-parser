use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};

use crate::bitcoin::sats_to_btc;

use super::{OneShotStates, UnrealizedState};

#[derive(Deref, DerefMut, Default, Debug)]
pub struct PriceInCentsToAmount(BTreeMap<u64, u64>);

impl PriceInCentsToAmount {
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

    pub fn compute_on_shot_states(
        &self,
        supply: u64,
        block_price: f32,
        date_price: Option<f32>,
    ) -> OneShotStates {
        let mut one_shot_states = OneShotStates::default();

        if date_price.is_some() {
            one_shot_states
                .unrealized_date_state
                .replace(UnrealizedState::default());
        }

        let mut processed_amount = 0;

        self.iter()
            .for_each(|(mean_price_paid_in_cent, sat_amount)| {
                processed_amount += sat_amount;

                let mean_price_paid = (*mean_price_paid_in_cent as f32) / 100.0;

                let btc_amount = sats_to_btc(*sat_amount);

                one_shot_states.price_paid_state.iterate(
                    mean_price_paid,
                    btc_amount,
                    *sat_amount,
                    supply,
                );

                one_shot_states.unrealized_block_state.iterate(
                    mean_price_paid,
                    block_price,
                    *sat_amount,
                    btc_amount,
                );

                if let Some(unrealized_date_state) = one_shot_states.unrealized_date_state.as_mut()
                {
                    unrealized_date_state.iterate(
                        mean_price_paid,
                        date_price.unwrap(),
                        *sat_amount,
                        btc_amount,
                    );
                }
            });

        if processed_amount != supply {
            dbg!(processed_amount, supply);
            panic!("processed_amount isn't equal to supply")
        }

        one_shot_states
    }
}
