use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};

use crate::{
    actions::ReceivedData,
    bitcoin::sats_to_btc,
    parse::BlockPath,
    states::{DateDataVec, OutputState},
    utils::{difference_in_days_between_timestamps, timestamp_to_year},
};

use super::SplitByUTXOCohort;

#[derive(Deref, DerefMut, Default)]
pub struct UTXOCohortsReceivedStates(SplitByUTXOCohort<OutputState>);

impl UTXOCohortsReceivedStates {
    pub fn compute(
        &mut self,
        date_data_vec: &DateDataVec,
        block_path_to_received_data: BTreeMap<BlockPath, ReceivedData>,
    ) {
        if let Some(last_date_data) = date_data_vec.last() {
            let last_block_data = last_date_data.blocks.last().unwrap();

            block_path_to_received_data
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
                .for_each(|(block_data, received_data)| {
                    let days_old = difference_in_days_between_timestamps(
                        block_data.timestamp,
                        last_block_data.timestamp,
                    );

                    let year = timestamp_to_year(block_data.timestamp);

                    let volume = sats_to_btc(received_data.volume);

                    self.filtered_apply(&days_old, &year, |state| {
                        state.iterate(received_data.count as f32, volume);
                    });
                })
        }
    }
}
