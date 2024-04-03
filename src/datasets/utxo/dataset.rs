use itertools::Itertools;

use crate::{
    bitcoin::sats_to_btc,
    datasets::{
        AnyDataset, AnyDatasetGroup, ExportData, GenericDataset, InputState, MinInitialState,
        OutputState, PricePaidState, ProcessedBlockData, RealizedState, SubDataset, SupplyState,
        UTXOState, UnrealizedState,
    },
    parse::{reverse_date_index, AnyBiMap, AnyDateMap, AnyHeightMap, BlockData},
};

use super::UTXOFilter;

pub struct UTXODataset {
    min_initial_state: MinInitialState,

    filter: UTXOFilter,

    pub subs: SubDataset,
}

impl UTXODataset {
    pub fn import(
        parent_path: &str,
        name: Option<&str>,
        range: UTXOFilter,
    ) -> color_eyre::Result<Self> {
        let folder_path = {
            if let Some(name) = name {
                format!("{parent_path}/{name}")
            } else {
                parent_path.to_owned()
            }
        };

        let s = Self {
            min_initial_state: MinInitialState::default(),

            filter: range,

            subs: SubDataset::import(&folder_path)?,
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl GenericDataset for UTXODataset {
    fn insert_block_data(&self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            block_path_to_received_data,
            block_path_to_spent_data,
            block_price,
            is_date_last_block,
            date,
            height,
            date_price,
            states,
            ..
        } = processed_block_data;

        let date_data_vec = &states.date_data_vec;

        let mut supply_state = SupplyState::default();
        let mut utxo_state = UTXOState::default();
        let mut input_state = InputState::default();
        let mut output_state = OutputState::default();
        let mut pp_state = PricePaidState::default();
        let mut realized_state = RealizedState::default();
        let mut unrealized_height_state = UnrealizedState::default();
        let mut unrealized_date_state = UnrealizedState::default();

        let date_data_vec_len = states.date_data_vec.len() as u16;

        let needs_price_paid_data = self.subs.price_paid.should_insert(height, date);
        let needs_unrealized_data = self.subs.unrealized.should_insert(height, date);
        let needs_realized_data = self.subs.realized.should_insert(height, date);
        let needs_input_data = self.subs.input.should_insert(height, date);
        let needs_output_data = self.subs.output.should_insert(height, date);

        date_data_vec
            .iter()
            .filter(|date_data| {
                self.filter
                    .check(&date_data.reverse_index(date_data_vec_len), &date_data.year)
            })
            .flat_map(|date_data| &date_data.blocks)
            .for_each(|block_data| {
                let price = block_data.price;
                let sat_amount = block_data.amount;
                let btc_amount = sats_to_btc(sat_amount);

                supply_state.supply += sat_amount;
                utxo_state.count += block_data.spendable_outputs as usize;

                if needs_unrealized_data {
                    unrealized_height_state.iterate(price, block_price, sat_amount, btc_amount);

                    if is_date_last_block {
                        unrealized_date_state.iterate(price, date_price, sat_amount, btc_amount);
                    }
                }
            });

        let total_supply = supply_state.supply;

        self.subs.supply.insert(processed_block_data, &supply_state);

        self.subs.utxo.insert(processed_block_data, &utxo_state);

        if needs_unrealized_data {
            self.subs.unrealized.insert(
                processed_block_data,
                &unrealized_height_state,
                &unrealized_date_state,
            );
        }

        if needs_price_paid_data {
            let len = states.date_data_vec.len() as u16;

            states
                .price_to_block_path
                .values()
                .map(|block_path| {
                    (
                        states
                            .date_data_vec
                            .get(block_path.date_index as usize)
                            .unwrap(),
                        block_path.block_index,
                    )
                })
                .filter(|(date_data, _)| {
                    self.filter
                        .check(&date_data.reverse_index(len), &date_data.year)
                })
                .map(|(date_data, block_index)| date_data.blocks.get(block_index as usize).unwrap())
                .for_each(|block_data| {
                    let price = block_data.price;
                    let sat_amount = block_data.amount;
                    let btc_amount = sats_to_btc(sat_amount);

                    pp_state.iterate(price, btc_amount, sat_amount, total_supply);
                });

            // MUST BE after insert supply
            self.subs
                .price_paid
                .insert(processed_block_data, &pp_state, &self.subs.supply.total);
        }

        if needs_output_data {
            block_path_to_received_data
                .iter()
                .map(|(block_path, data)| {
                    let date_data = date_data_vec.get(block_path.date_index as usize).unwrap();
                    (block_path, date_data, data)
                })
                .filter(|(block_path, date_data, _)| {
                    self.filter.check(
                        &reverse_date_index(block_path.date_index, date_data_vec_len),
                        &date_data.year,
                    )
                })
                .for_each(|(_, _, data)| {
                    output_state.iterate(data.count as f32, sats_to_btc(data.volume));
                });

            self.subs.output.insert(processed_block_data, &output_state);
        }

        if needs_realized_data || needs_input_data {
            block_path_to_spent_data
                .iter()
                .map(|(block_path, data)| {
                    let date_data = date_data_vec.get(block_path.date_index as usize).unwrap();
                    (block_path, date_data, data)
                })
                .filter(|(block_path, date_data, _)| {
                    self.filter.check(
                        &reverse_date_index(block_path.date_index, date_data_vec_len),
                        &date_data.year,
                    )
                })
                .for_each(|(block_path, date_data, spent_value)| {
                    let btc_spent = sats_to_btc(spent_value.volume);

                    if needs_input_data {
                        input_state.iterate(spent_value.count as f32, btc_spent);
                    }

                    if needs_realized_data {
                        let BlockData {
                            price: previous_price,
                            ..
                        } = date_data
                            .blocks
                            .get(block_path.block_index as usize)
                            .unwrap();

                        let previous_dollar_amount = *previous_price * btc_spent;
                        let current_dollar_amount = block_price * btc_spent;

                        if previous_dollar_amount < current_dollar_amount {
                            realized_state.realized_profit +=
                                current_dollar_amount - previous_dollar_amount
                        } else if current_dollar_amount < previous_dollar_amount {
                            realized_state.realized_loss +=
                                previous_dollar_amount - current_dollar_amount
                        }
                    }
                });

            if needs_realized_data {
                self.subs
                    .realized
                    .insert(processed_block_data, &realized_state);
            }

            if needs_input_data {
                self.subs.input.insert(processed_block_data, &input_state);
            }
        }
    }
}

impl AnyDataset for UTXODataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn prepare(&self, export_data: &ExportData) {
        self.subs
            .to_vec()
            .into_iter()
            .for_each(|d| d.prepare(export_data))
    }

    fn compute(&self, export_data: &ExportData) {
        self.subs
            .to_vec()
            .into_iter()
            .for_each(|d| d.compute(export_data))
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_inserted_height_map_vec())
            .collect_vec()
    }

    fn to_any_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_inserted_date_map_vec())
            .collect_vec()
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_bi_map_vec())
            .collect_vec()
    }

    fn to_any_exported_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_date_map_vec())
            .collect_vec()
    }

    fn to_any_exported_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_height_map_vec())
            .collect_vec()
    }
}
