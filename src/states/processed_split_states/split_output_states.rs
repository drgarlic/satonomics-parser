use derive_deref::{Deref, DerefMut};

use crate::{
    datasets::OutputState,
    parse::{
        AddressRealizedData, LiquidityClassification, RawAddressSize, RawAddressSplit,
        SplitByLiquidity,
    },
};

use super::SplitByCohort;

#[derive(Deref, DerefMut, Default)]
pub struct SplitOutputStates(SplitByCohort<SplitByLiquidity<OutputState>>);

impl SplitOutputStates {
    pub fn iterate_output(
        &mut self,
        realized_data: &AddressRealizedData,
        liquidity_classification: &LiquidityClassification,
    ) {
        let count = realized_data.utxos_created as f32;
        let volume = realized_data.received as f32;

        let split_count = liquidity_classification.split(count);
        let split_volume = liquidity_classification.split(volume);

        let iterate = move |state: &mut SplitByLiquidity<OutputState>| {
            state.all.iterate(count, volume);

            state
                .illiquid
                .iterate(split_count.illiquid, split_volume.illiquid);

            state
                .liquid
                .iterate(split_count.liquid, split_volume.liquid);

            state
                .highly_liquid
                .iterate(split_count.highly_liquid, split_volume.highly_liquid);
        };

        if let Some(state) = self.get_mut_state(&RawAddressSplit::Type(
            realized_data.initial_address_data.address_type,
        )) {
            iterate(state);
        }

        if let Some(state) = self.get_mut_state(&RawAddressSplit::Size(
            RawAddressSize::from_amount(realized_data.initial_address_data.amount),
        )) {
            iterate(state);
        }
    }
}
