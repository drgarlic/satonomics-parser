use derive_deref::{Deref, DerefMut};

use crate::{
    datasets::RealizedState,
    parse::{
        AddressRealizedData, LiquidityClassification, RawAddressSize, RawAddressSplit,
        SplitByLiquidity,
    },
};

use super::SplitByCohort;

#[derive(Deref, DerefMut, Default)]
pub struct SplitRealizedStates(SplitByCohort<SplitByLiquidity<RealizedState>>);

impl SplitRealizedStates {
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
