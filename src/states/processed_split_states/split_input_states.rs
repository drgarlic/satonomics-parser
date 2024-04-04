use derive_deref::{Deref, DerefMut};

use crate::{
    datasets::InputState,
    parse::{AddressRealizedData, LiquidityClassification, SplitByLiquidity},
};

use super::SplitByCohort;

#[derive(Deref, DerefMut, Default)]
pub struct SplitInputStates(SplitByCohort<SplitByLiquidity<InputState>>);

impl SplitInputStates {
    pub fn iterate_input(
        &mut self,
        realized_data: &AddressRealizedData,
        liquidity_classification: &LiquidityClassification,
    ) {
        let count = realized_data.utxos_destroyed as f32;
        let volume = realized_data.sent as f32;

        let split_count = liquidity_classification.split(count);
        let split_volume = liquidity_classification.split(volume);

        let iterate = move |state: &mut SplitByLiquidity<InputState>| {
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

        self.iterate(&realized_data.initial_address_data, &iterate);
    }
}
