use crate::datasets::{SupplyState, UTXOState};

use super::{MeanPricePaidInCentsToAmount, OneShotStates};

#[derive(Default, Debug)]
pub struct ProcessedAddressesState {
    mean_price_paid_in_cents_to_amount: MeanPricePaidInCentsToAmount,
    pub supply_state: SupplyState,
    pub utxo_state: UTXOState,
}

impl ProcessedAddressesState {
    pub fn increment(&mut self, amount: u64, utxo_count: usize, mean_price_paid_in_cents: u64) {
        if amount == 0 {
            if utxo_count != 0 {
                unreachable!("Shouldn't be possible")
            }
            return;
        }

        self.supply_state.increment(amount);
        self.utxo_state.increment(utxo_count);
        self.mean_price_paid_in_cents_to_amount
            .increment(mean_price_paid_in_cents, amount);
    }

    pub fn decrement(&mut self, amount: u64, utxo_count: usize, mean_price_paid_in_cents: u64) {
        if amount == 0 {
            if utxo_count != 0 {
                unreachable!("Shouldn't be possible")
            }
            return;
        }

        self.supply_state.decrement(amount);
        self.utxo_state.decrement(utxo_count);
        self.mean_price_paid_in_cents_to_amount
            .decrement(mean_price_paid_in_cents, amount);
    }

    pub fn compute_one_shot_states(
        &self,
        block_price: f32,
        date_price: Option<f32>,
    ) -> OneShotStates {
        self.mean_price_paid_in_cents_to_amount
            .compute_on_shot_states(self.supply_state.supply, block_price, date_price)
    }
}
