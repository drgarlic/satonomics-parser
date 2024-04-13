use derive_deref::{Deref, DerefMut};

use crate::{
    parse::{AddressRealizedData, LiquidityClassification, SplitByLiquidity},
    states::RealizedState,
};

use super::SplitByAddressCohort;

#[derive(Deref, DerefMut, Default)]
pub struct AddressCohortsRealizedStates(SplitByAddressCohort<SplitByLiquidity<RealizedState>>);

impl AddressCohortsRealizedStates {
    pub fn iterate_realized(
        &mut self,
        realized_data: &AddressRealizedData,
        liquidity_classification: &LiquidityClassification,
    ) {
        let profit = realized_data.profit;
        let loss = realized_data.loss;

        let split_profit = liquidity_classification.split(profit);
        let split_loss = liquidity_classification.split(loss);

        let iterate = move |state: &mut SplitByLiquidity<RealizedState>| {
            state.all.iterate(profit, loss);

            state
                .illiquid
                .iterate(split_profit.illiquid, split_loss.illiquid);

            state.liquid.iterate(split_profit.liquid, split_loss.liquid);

            state
                .highly_liquid
                .iterate(split_profit.highly_liquid, split_loss.highly_liquid);
        };

        self.iterate(&realized_data.initial_address_data, iterate);
    }
}
