use chrono::Datelike;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::slice::ParallelSliceMut;

use crate::{
    bitcoin::sats_to_btc,
    datasets::{
        AnyDataset, PricePaidState, PricePaidSubDataset, ProcessedBlockData, RealizedSubDataset,
        SupplySubDataset, UTXOsMetadataSubDataset, UnrealizedState, UnrealizedSubDataset,
    },
    structs::{AnyDateMap, BlockPath},
    structs::{AnyHeightMap, BlockData},
};

use super::UTXOFilter;

pub struct UTXODataset {
    filter: UTXOFilter,
    price_paid: PricePaidSubDataset,
    realized: RealizedSubDataset,
    supply: SupplySubDataset,
    unrealized: UnrealizedSubDataset,
    utxos_metadata: UTXOsMetadataSubDataset,
}

impl UTXODataset {
    pub fn import(parent_path: &str, name: &str, range: UTXOFilter) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/{name}");
        Ok(Self {
            filter: range,
            price_paid: PricePaidSubDataset::import(&folder_path)?,
            realized: RealizedSubDataset::import(&folder_path)?,
            supply: SupplySubDataset::import(&folder_path)?,
            unrealized: UnrealizedSubDataset::import(&folder_path)?,
            utxos_metadata: UTXOsMetadataSubDataset::import(&folder_path)?,
        })
    }
}

impl AnyDataset for UTXODataset {
    fn insert_block_data(&self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            block_path_to_spent_value,
            block_price,
            is_date_last_block,
            date,
            height,
            date_price,
            states,
            ..
        } = processed_block_data;

        let date_data_vec = &states.date_data_vec;

        let len = date_data_vec.len();

        let mut total_supply = 0;
        let mut utxo_count = 0;

        let mut vec = {
            match self.filter {
                UTXOFilter::Full => date_data_vec.iter(),
                UTXOFilter::From(from) if from < len => date_data_vec[..(len - from)].iter(),
                UTXOFilter::To(to) => {
                    if to <= len {
                        date_data_vec[(len - to)..].iter()
                    } else {
                        date_data_vec.iter()
                    }
                }
                UTXOFilter::FromTo { from, to } if from < len => {
                    if to <= len {
                        date_data_vec[(len - to)..(len - from)].iter()
                    } else {
                        date_data_vec[..(len - from)].iter()
                    }
                }
                UTXOFilter::Year(_) => date_data_vec.iter(),
                _ => date_data_vec[..0].iter(),
            }
        }
        // Can't figure how to put the filter inside the match without type issues
        .filter(|date_data| {
            if let UTXOFilter::Year(year) = self.filter {
                date_data.date.year() == year as i32
            } else {
                true
            }
        })
        .flat_map(|date_data| &date_data.blocks)
        .map(|block_data| {
            let sat_amount = block_data.amount;

            total_supply += sat_amount;

            utxo_count += block_data.spendable_outputs as usize;

            (OrderedFloat(block_data.price), sat_amount)
        })
        .collect_vec();

        vec.par_sort_unstable_by(|a, b| Ord::cmp(&a.0, &b.0));

        let total_supply_in_btc = sats_to_btc(total_supply);

        let mut pp_state = PricePaidState::default();

        let mut unrealized_height_state = UnrealizedState::default();

        let mut unrealized_date_state = {
            if is_date_last_block {
                Some(UnrealizedState::default())
            } else {
                None
            }
        };

        self.supply.insert(processed_block_data, total_supply);

        self.utxos_metadata.insert(processed_block_data, utxo_count);

        if !self.price_paid.are_date_and_height_safe(date, height)
            || !self.unrealized.are_date_and_height_safe(date, height)
        {
            vec.iter().for_each(|(price, sat_amount)| {
                let price = price.0;
                let sat_amount = *sat_amount;

                let btc_amount = sats_to_btc(sat_amount);

                pp_state.iterate(price, btc_amount, sat_amount, total_supply);

                unrealized_height_state.iterate(price, block_price, sat_amount, btc_amount);

                if is_date_last_block {
                    unrealized_date_state
                        .as_mut()
                        .unwrap()
                        .iterate(price, date_price, sat_amount, btc_amount);
                }
            });

            self.price_paid
                .insert(processed_block_data, pp_state, total_supply_in_btc);

            self.unrealized.insert(
                processed_block_data,
                unrealized_height_state,
                unrealized_date_state,
            );
        }

        if !self.realized.are_date_and_height_safe(date, height) {
            let mut realized_profit = 0.0;
            let mut realized_loss = 0.0;

            block_path_to_spent_value
                .iter()
                .map(|(block_path, value)| {
                    let date_data = date_data_vec.get(block_path.date_index as usize).unwrap();
                    (block_path, date_data, value)
                })
                .filter(|(block_path, date_data, _)| {
                    let diff = len - 1 - (block_path.date_index as usize);

                    match self.filter {
                        UTXOFilter::Full => true,
                        UTXOFilter::From(from) => from <= diff,
                        UTXOFilter::To(to) => to > diff,
                        UTXOFilter::FromTo { from, to } => from <= diff && to > diff,
                        UTXOFilter::Year(year) => year == date_data.date.year() as usize,
                    }
                })
                .for_each(|(block_path, date_data, value)| {
                    let BlockData {
                        price: previous_price,
                        ..
                    } = date_data
                        .blocks
                        .get(block_path.block_index as usize)
                        .unwrap();

                    let previous_dollar_amount = *previous_price as f64 * sats_to_btc(*value);
                    let current_dollar_amount = block_price as f64 * sats_to_btc(*value);

                    if previous_dollar_amount < current_dollar_amount {
                        realized_profit += (current_dollar_amount - previous_dollar_amount) as f32
                    } else if current_dollar_amount < previous_dollar_amount {
                        realized_loss += (previous_dollar_amount - current_dollar_amount) as f32
                    }
                });

            self.realized
                .insert(processed_block_data, realized_loss, realized_profit);
        }
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        [
            self.price_paid.to_any_height_map_vec(),
            self.realized.to_any_height_map_vec(),
            self.supply.to_any_height_map_vec(),
            self.unrealized.to_any_height_map_vec(),
            self.utxos_metadata.to_any_height_map_vec(),
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        [
            self.price_paid.to_any_date_map_vec(),
            self.realized.to_any_date_map_vec(),
            self.supply.to_any_date_map_vec(),
            self.unrealized.to_any_date_map_vec(),
            self.utxos_metadata.to_any_date_map_vec(),
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }
}
