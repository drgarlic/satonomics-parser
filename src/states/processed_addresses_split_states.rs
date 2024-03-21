use std::{collections::BTreeMap, thread};

use derive_deref::{Deref, DerefMut};

use crate::{
    bitcoin::sats_to_btc,
    datasets::{PricePaidState, RealizedState, SupplyState, UTXOsMetadataState, UnrealizedState},
    parse::{
        AddressData, AddressRealizedData, LiquiditySplitResult, RawAddressSize, RawAddressSplit,
        RawAddressType, SplitByLiquidity,
    },
};

use super::AddressIndexToAddressData;

#[derive(Deref, DerefMut, Default, Debug)]
struct MeanPricePaidInCentsToAmount(BTreeMap<u64, u64>);

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

        // TODO: Try par_iter + reduce
        self.iter()
            .for_each(|(mean_price_paid_in_cent, sat_amount)| {
                let mean_price_paid = (*mean_price_paid_in_cent as f32) / 100.0;
                let btc_amount = sats_to_btc(*sat_amount);
                unrealized_state.iterate(mean_price_paid, ref_price, *sat_amount, btc_amount);
            });

        unrealized_state
    }
}

#[derive(Default, Debug)]
pub struct ProcessedAddressesState {
    mean_price_paid_in_cents_to_amount: MeanPricePaidInCentsToAmount,
    pub supply: SupplyState,
    pub utxos_metadata: UTXOsMetadataState,
}

impl ProcessedAddressesState {
    pub fn increment(&mut self, amount: u64, utxo_count: usize, mean_price_paid_in_cents: u64) {
        if amount == 0 {
            if utxo_count != 0 {
                unreachable!("Shouldn't be possible")
            }
            return;
        }

        self.supply.increment(amount);
        self.utxos_metadata.increment(utxo_count);
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

        self.supply.decrement(amount);
        self.utxos_metadata.decrement(utxo_count);
        self.mean_price_paid_in_cents_to_amount
            .decrement(mean_price_paid_in_cents, amount);
    }

    pub fn compute_price_paid_state(&self) -> PricePaidState {
        self.mean_price_paid_in_cents_to_amount
            .compute_price_paid_state(self.supply.total_supply)
    }

    pub fn compute_unrealized_state(&self, ref_price: f32) -> UnrealizedState {
        self.mean_price_paid_in_cents_to_amount
            .compute_unrealized_state(ref_price)
    }
}

#[derive(Default)]
pub struct LiquiditySplitProcessedAddressState {
    pub address_count: usize,
    pub split: SplitByLiquidity<ProcessedAddressesState>,
}

impl LiquiditySplitProcessedAddressState {
    pub fn increment(
        &mut self,
        amount: u64,
        utxo_count: usize,
        mean_price_paid_in_cents: u64,
        split_sat_amount: &LiquiditySplitResult,
        split_utxo_count: &LiquiditySplitResult,
    ) {
        self.address_count += 1;

        self.split
            .all
            .increment(amount, utxo_count, mean_price_paid_in_cents);
        self.split.illiquid.increment(
            split_sat_amount.illiquid.round() as u64,
            split_utxo_count.illiquid.round() as usize,
            mean_price_paid_in_cents,
        );
        self.split.liquid.increment(
            split_sat_amount.liquid.round() as u64,
            split_utxo_count.liquid.round() as usize,
            mean_price_paid_in_cents,
        );
        self.split.highly_liquid.increment(
            split_sat_amount.highly_liquid.round() as u64,
            split_utxo_count.highly_liquid.round() as usize,
            mean_price_paid_in_cents,
        );
    }

    pub fn decrement(
        &mut self,
        amount: u64,
        utxo_count: usize,
        mean_price_paid_in_cents: u64,
        split_sat_amount: &LiquiditySplitResult,
        split_utxo_count: &LiquiditySplitResult,
    ) {
        self.address_count -= 1;

        self.split
            .all
            .decrement(amount, utxo_count, mean_price_paid_in_cents);
        self.split.illiquid.decrement(
            split_sat_amount.illiquid.round() as u64,
            split_utxo_count.illiquid.round() as usize,
            mean_price_paid_in_cents,
        );
        self.split.liquid.decrement(
            split_sat_amount.liquid.round() as u64,
            split_utxo_count.liquid.round() as usize,
            mean_price_paid_in_cents,
        );
        self.split.highly_liquid.decrement(
            split_sat_amount.highly_liquid.round() as u64,
            split_utxo_count.highly_liquid.round() as usize,
            mean_price_paid_in_cents,
        );
    }

    pub fn compute_price_paid_state(&self) -> SplitByLiquidity<PricePaidState> {
        thread::scope(|scope| {
            let all_handle = scope.spawn(|| self.split.all.compute_price_paid_state());
            let illiquid_handle = scope.spawn(|| self.split.illiquid.compute_price_paid_state());
            let liquid_handle = scope.spawn(|| self.split.liquid.compute_price_paid_state());
            let highly_liquid_handle =
                scope.spawn(|| self.split.highly_liquid.compute_price_paid_state());

            SplitByLiquidity {
                all: all_handle.join().unwrap(),
                illiquid: illiquid_handle.join().unwrap(),
                liquid: liquid_handle.join().unwrap(),
                highly_liquid: highly_liquid_handle.join().unwrap(),
            }
        })
    }

    pub fn compute_unrealized_state(&self, ref_price: f32) -> SplitByLiquidity<UnrealizedState> {
        thread::scope(|scope| {
            let all_handle = scope.spawn(|| self.split.all.compute_unrealized_state(ref_price));
            let illiquid_handle =
                scope.spawn(|| self.split.illiquid.compute_unrealized_state(ref_price));
            let liquid_handle =
                scope.spawn(|| self.split.liquid.compute_unrealized_state(ref_price));
            let highly_liquid_handle =
                scope.spawn(|| self.split.highly_liquid.compute_unrealized_state(ref_price));

            SplitByLiquidity {
                all: all_handle.join().unwrap(),
                illiquid: illiquid_handle.join().unwrap(),
                liquid: liquid_handle.join().unwrap(),
                highly_liquid: highly_liquid_handle.join().unwrap(),
            }
        })
    }
}

#[derive(Default)]
pub struct SplitByCohort<T> {
    plankton: T,
    shrimp: T,
    crab: T,
    fish: T,
    shark: T,
    whale: T,
    humpback: T,
    megalodon: T,

    p2pk: T,
    p2pkh: T,
    p2sh: T,
    p2wpkh: T,
    p2wsh: T,
    p2tr: T,
}

impl<T> SplitByCohort<T> {
    pub fn get_state(&self, split: &RawAddressSplit) -> Option<&T> {
        match &split {
            RawAddressSplit::Type(address_type) => match address_type {
                RawAddressType::P2PK => Some(&self.p2pk),
                RawAddressType::P2PKH => Some(&self.p2pkh),
                RawAddressType::P2SH => Some(&self.p2sh),
                RawAddressType::P2WPKH => Some(&self.p2wpkh),
                RawAddressType::P2WSH => Some(&self.p2wsh),
                RawAddressType::P2TR => Some(&self.p2tr),
                _ => None,
            },
            RawAddressSplit::Size(address_size) => match address_size {
                RawAddressSize::Empty => None,
                RawAddressSize::Plankton => Some(&self.plankton),
                RawAddressSize::Shrimp => Some(&self.shrimp),
                RawAddressSize::Crab => Some(&self.crab),
                RawAddressSize::Fish => Some(&self.fish),
                RawAddressSize::Shark => Some(&self.shark),
                RawAddressSize::Whale => Some(&self.whale),
                RawAddressSize::Humpback => Some(&self.humpback),
                RawAddressSize::Megalodon => Some(&self.megalodon),
            },
        }
    }

    fn get_mut_state(&mut self, split: &RawAddressSplit) -> Option<&mut T> {
        match &split {
            RawAddressSplit::Type(address_type) => match address_type {
                RawAddressType::P2PK => Some(&mut self.p2pk),
                RawAddressType::P2PKH => Some(&mut self.p2pkh),
                RawAddressType::P2SH => Some(&mut self.p2sh),
                RawAddressType::P2WPKH => Some(&mut self.p2wpkh),
                RawAddressType::P2WSH => Some(&mut self.p2wsh),
                RawAddressType::P2TR => Some(&mut self.p2tr),
                _ => None,
            },
            RawAddressSplit::Size(address_size) => match address_size {
                RawAddressSize::Empty => None,
                RawAddressSize::Plankton => Some(&mut self.plankton),
                RawAddressSize::Shrimp => Some(&mut self.shrimp),
                RawAddressSize::Crab => Some(&mut self.crab),
                RawAddressSize::Fish => Some(&mut self.fish),
                RawAddressSize::Shark => Some(&mut self.shark),
                RawAddressSize::Whale => Some(&mut self.whale),
                RawAddressSize::Humpback => Some(&mut self.humpback),
                RawAddressSize::Megalodon => Some(&mut self.megalodon),
            },
        }
    }
}

#[derive(Deref, DerefMut, Default)]
pub struct SplitRealizedStates(SplitByCohort<SplitByLiquidity<RealizedState>>);

#[derive(Deref, DerefMut, Default)]
pub struct SplitUnrealizedStates(SplitByCohort<SplitByLiquidity<UnrealizedState>>);

#[derive(Deref, DerefMut, Default)]
pub struct SplitPricePaidStates(SplitByCohort<SplitByLiquidity<PricePaidState>>);

impl SplitRealizedStates {
    pub fn iterate_realized(&mut self, address_realized_data: &AddressRealizedData) {
        let profit = address_realized_data.profit;
        let loss = address_realized_data.loss;

        // Realized == previous amount
        // If a whale sent all its sats to another address at a loss, it's the whale that realized the loss not the empty adress
        let liquidity_classification = address_realized_data
            .initial_address_data
            .compute_liquidity_classification();

        let split_profit = liquidity_classification.split(profit as f64);
        let split_loss = liquidity_classification.split(loss as f64);

        let iterate = move |state: &mut SplitByLiquidity<RealizedState>| {
            state.all.iterate(profit, loss);
            state
                .illiquid
                .iterate(split_profit.illiquid as f32, split_loss.illiquid as f32);
            state
                .liquid
                .iterate(split_profit.liquid as f32, split_loss.liquid as f32);
            state.highly_liquid.iterate(
                split_profit.highly_liquid as f32,
                split_loss.highly_liquid as f32,
            );
        };

        if let Some(state) = self.get_mut_state(&RawAddressSplit::Type(
            address_realized_data.initial_address_data.address_type,
        )) {
            iterate(state);
        }

        if let Some(state) = self.get_mut_state(&RawAddressSplit::Size(
            RawAddressSize::from_amount(address_realized_data.initial_address_data.amount),
        )) {
            iterate(state);
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct SplitVariousAddressStates(SplitByCohort<LiquiditySplitProcessedAddressState>);

impl SplitVariousAddressStates {
    pub fn init(address_index_to_address_data: &AddressIndexToAddressData) -> Self {
        let mut this = Self::default();

        address_index_to_address_data
            .iter()
            .for_each(|(_, address_data)| this.increment(address_data));

        this
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

        let split_sat_amount_amount = liquidity_classification.split(amount as f64);
        let split_utxo_count = liquidity_classification.split(utxo_count as f64);

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
