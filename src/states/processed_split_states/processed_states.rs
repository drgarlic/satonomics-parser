use crate::datasets::{PricePaidState, SupplyState, UTXOState, UnrealizedState};

use super::MeanPricePaidInCentsToAmount;

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

    pub fn compute_price_paid_state(&self) -> PricePaidState {
        self.mean_price_paid_in_cents_to_amount
            .compute_price_paid_state(self.supply_state.supply)
    }

    pub fn compute_unrealized_state(&self, ref_price: f32) -> UnrealizedState {
        self.mean_price_paid_in_cents_to_amount
            .compute_unrealized_state(ref_price)
    }
}
