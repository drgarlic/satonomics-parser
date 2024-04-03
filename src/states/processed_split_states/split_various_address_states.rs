use std::thread;

use derive_deref::{Deref, DerefMut};

use crate::{
    parse::{AddressData, AddressRealizedData, RawAddressSize, RawAddressSplit},
    states::AddressIndexToAddressData,
};

use super::{
    LiquiditySplitProcessedAddressState, SplitByCohort, SplitPricePaidStates, SplitUnrealizedStates,
};

#[derive(Default, Deref, DerefMut)]
pub struct SplitVariousAddressStates(SplitByCohort<LiquiditySplitProcessedAddressState>);

impl SplitVariousAddressStates {
    pub fn init(address_index_to_address_data: &AddressIndexToAddressData) -> Self {
        let mut s = Self::default();

        address_index_to_address_data
            .iter()
            .for_each(|(_, address_data)| s.increment(address_data));

        s
    }

    pub fn iterate(
        &mut self,
        address_realized_data: &AddressRealizedData,
        current_address_data: &AddressData,
    ) {
        self.decrement(&address_realized_data.initial_address_data);
        self.increment(current_address_data);
    }

    pub fn compute_price_paid_states(&mut self) -> SplitPricePaidStates {
        thread::scope(|scope| {
            let plankton_handle = scope.spawn(|| self.plankton.compute_price_paid_state());
            let shrimp_handle = scope.spawn(|| self.shrimp.compute_price_paid_state());
            let crab_handle = scope.spawn(|| self.crab.compute_price_paid_state());
            let fish_handle = scope.spawn(|| self.fish.compute_price_paid_state());
            let shark_handle = scope.spawn(|| self.shark.compute_price_paid_state());
            let whale_handle = scope.spawn(|| self.whale.compute_price_paid_state());
            let humpback_handle = scope.spawn(|| self.humpback.compute_price_paid_state());
            let megalodon_handle = scope.spawn(|| self.megalodon.compute_price_paid_state());

            let p2pk_handle = scope.spawn(|| self.p2pk.compute_price_paid_state());
            let p2pkh_handle = scope.spawn(|| self.p2pkh.compute_price_paid_state());
            let p2sh_handle = scope.spawn(|| self.p2sh.compute_price_paid_state());
            let p2wpkh_handle = scope.spawn(|| self.p2wpkh.compute_price_paid_state());
            let p2wsh_handle = scope.spawn(|| self.p2wsh.compute_price_paid_state());
            let p2tr_handle = scope.spawn(|| self.p2tr.compute_price_paid_state());

            SplitPricePaidStates(SplitByCohort {
                plankton: plankton_handle.join().unwrap(),
                shrimp: shrimp_handle.join().unwrap(),
                crab: crab_handle.join().unwrap(),
                fish: fish_handle.join().unwrap(),
                shark: shark_handle.join().unwrap(),
                whale: whale_handle.join().unwrap(),
                humpback: humpback_handle.join().unwrap(),
                megalodon: megalodon_handle.join().unwrap(),

                p2pk: p2pk_handle.join().unwrap(),
                p2pkh: p2pkh_handle.join().unwrap(),
                p2sh: p2sh_handle.join().unwrap(),
                p2wpkh: p2wpkh_handle.join().unwrap(),
                p2wsh: p2wsh_handle.join().unwrap(),
                p2tr: p2tr_handle.join().unwrap(),
            })
        })
    }

    pub fn compute_unrealized_states(&mut self, ref_price: f32) -> SplitUnrealizedStates {
        thread::scope(|scope| {
            let plankton_handle = scope.spawn(|| self.plankton.compute_unrealized_state(ref_price));
            let shrimp_handle = scope.spawn(|| self.shrimp.compute_unrealized_state(ref_price));
            let crab_handle = scope.spawn(|| self.crab.compute_unrealized_state(ref_price));
            let fish_handle = scope.spawn(|| self.fish.compute_unrealized_state(ref_price));
            let shark_handle = scope.spawn(|| self.shark.compute_unrealized_state(ref_price));
            let whale_handle = scope.spawn(|| self.whale.compute_unrealized_state(ref_price));
            let humpback_handle = scope.spawn(|| self.humpback.compute_unrealized_state(ref_price));
            let megalodon_handle =
                scope.spawn(|| self.megalodon.compute_unrealized_state(ref_price));

            let p2pk_handle = scope.spawn(|| self.p2pk.compute_unrealized_state(ref_price));
            let p2pkh_handle = scope.spawn(|| self.p2pkh.compute_unrealized_state(ref_price));
            let p2sh_handle = scope.spawn(|| self.p2sh.compute_unrealized_state(ref_price));
            let p2wpkh_handle = scope.spawn(|| self.p2wpkh.compute_unrealized_state(ref_price));
            let p2wsh_handle = scope.spawn(|| self.p2wsh.compute_unrealized_state(ref_price));
            let p2tr_handle = scope.spawn(|| self.p2tr.compute_unrealized_state(ref_price));

            SplitUnrealizedStates(SplitByCohort {
                plankton: plankton_handle.join().unwrap(),
                shrimp: shrimp_handle.join().unwrap(),
                crab: crab_handle.join().unwrap(),
                fish: fish_handle.join().unwrap(),
                shark: shark_handle.join().unwrap(),
                whale: whale_handle.join().unwrap(),
                humpback: humpback_handle.join().unwrap(),
                megalodon: megalodon_handle.join().unwrap(),

                p2pk: p2pk_handle.join().unwrap(),
                p2pkh: p2pkh_handle.join().unwrap(),
                p2sh: p2sh_handle.join().unwrap(),
                p2wpkh: p2wpkh_handle.join().unwrap(),
                p2wsh: p2wsh_handle.join().unwrap(),
                p2tr: p2tr_handle.join().unwrap(),
            })
        })
    }

    /// Should always increment using current address data state
    fn increment(&mut self, address_data: &AddressData) {
        self._crement(address_data, true)
    }

    /// Should always decrement using initial address data state
    fn decrement(&mut self, address_data: &AddressData) {
        self._crement(address_data, false)
    }

    fn _crement(&mut self, address_data: &AddressData, increment: bool) {
        let amount = address_data.amount;
        let utxo_count = address_data.outputs_len as usize;

        // No need to either insert or remove if 0
        if amount == 0 {
            return;
        }

        // Rounded after the {significant_digits} to have the smallest btree possible
        let mut mean_price_paid_in_cents = (address_data.mean_price_paid * 100.0) as u64;
        let ilog10 = mean_price_paid_in_cents.checked_ilog10().unwrap_or(0) as i32;
        let significant_digits = 4;
        if ilog10 >= significant_digits {
            let log_diff = ilog10 - significant_digits + 1;
            let pow = 10.0_f64.powi(log_diff);
            mean_price_paid_in_cents =
                ((mean_price_paid_in_cents as f64 / pow).round() * pow) as u64;
        }

        let liquidity_classification = address_data.compute_liquidity_classification();

        let split_sat_amount_amount = liquidity_classification.split(amount as f32);
        let split_utxo_count = liquidity_classification.split(utxo_count as f32);

        if let Some(state) = self.get_mut_state(&RawAddressSplit::Type(address_data.address_type)) {
            if increment {
                state.increment(
                    amount,
                    utxo_count,
                    mean_price_paid_in_cents,
                    &split_sat_amount_amount,
                    &split_utxo_count,
                );
            } else {
                state.decrement(
                    amount,
                    utxo_count,
                    mean_price_paid_in_cents,
                    &split_sat_amount_amount,
                    &split_utxo_count,
                )
            }
        }

        if let Some(state) =
            self.get_mut_state(&RawAddressSplit::Size(RawAddressSize::from_amount(amount)))
        {
            if increment {
                state.increment(
                    amount,
                    utxo_count,
                    mean_price_paid_in_cents,
                    &split_sat_amount_amount,
                    &split_utxo_count,
                );
            } else {
                state.decrement(
                    amount,
                    utxo_count,
                    mean_price_paid_in_cents,
                    &split_sat_amount_amount,
                    &split_utxo_count,
                )
            }
        }
    }
}
