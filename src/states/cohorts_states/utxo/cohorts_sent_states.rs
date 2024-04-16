use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};

use crate::{
    actions::SpentData,
    bitcoin::sats_to_btc,
    parse::BlockPath,
    states::{DateDataVec, InputState, RealizedState},
    utils::{difference_in_days_between_timestamps, timestamp_to_year},
};

use super::SplitByUTXOCohort;

#[derive(Default, Debug)]
pub struct SentState {
    pub input: InputState,
    pub realized: RealizedState,
}

#[derive(Deref, DerefMut, Default)]
pub struct UTXOCohortsSentStates(SplitByUTXOCohort<SentState>);

impl UTXOCohortsSentStates {
    pub fn compute(
        &mut self,
        date_data_vec: &DateDataVec,
        block_path_to_spent_data: BTreeMap<BlockPath, SpentData>,
        current_price: f32,
    ) {
        if let Some(last_date_data) = date_data_vec.last() {
            let last_block_data = last_date_data.blocks.last().unwrap();

            block_path_to_spent_data
                .into_iter()
                .map(|(block_path, data)| {
                    let block_data = date_data_vec
                        .get(block_path.date_index as usize)
                        .unwrap()
                        .blocks
                        .get(block_path.block_index as usize)
                        .unwrap();

                    (block_data, data)
                })
                .for_each(|(block_data, spent_data)| {
                    let days_old = difference_in_days_between_timestamps(
                        block_data.timestamp,
                        last_block_data.timestamp,
                    );

                    let year = timestamp_to_year(block_data.timestamp);

                    let previous_price = block_data.price;

                    let btc_spent = sats_to_btc(spent_data.volume);

                    self.filtered_apply(&days_old, &year, |state| {
                        state.input.iterate(spent_data.count as f32, btc_spent);

                        let previous_dollar_amount = previous_price * btc_spent;
                        let current_dollar_amount = current_price * btc_spent;

                        if previous_dollar_amount < current_dollar_amount {
                            state.realized.realized_profit +=
                                current_dollar_amount - previous_dollar_amount;
                        } else if current_dollar_amount < previous_dollar_amount {
                            state.realized.realized_loss +=
                                previous_dollar_amount - current_dollar_amount;
                        }
                    })
                })
        }
    }
}
