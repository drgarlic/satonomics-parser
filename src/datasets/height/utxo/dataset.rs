use std::fs;

use chrono::Datelike;
use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::{
    bitcoin::sats_to_btc,
    datasets::{
        height::{PricesSubDataset, PricesSubDatasetInsertData},
        AnyHeightDataset, ProcessedBlockData,
    },
    structs::BlockPath,
    structs::{AnyHeightMap, BlockData},
};

use super::UTXOFilter;

pub struct UTXODataset {
    filter: UTXOFilter,
    prices: PricesSubDataset,
}

impl UTXODataset {
    pub fn import(parent_path: &str, name: &str, range: UTXOFilter) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/{name}");

        fs::create_dir_all(&folder_path)?;

        Ok(Self {
            filter: range,
            prices: PricesSubDataset::import(&folder_path)?,
        })
    }
}

impl AnyHeightDataset for UTXODataset {
    fn insert(
        &self,
        &ProcessedBlockData {
            states,
            price,
            height,
            block_path_to_spent_value,
            ..
        }: &ProcessedBlockData,
    ) {
        let date_data_vec = &states.date_data_vec;

        let len = date_data_vec.len();

        let mut realized_profit = 0.0;
        let mut realized_loss = 0.0;

        block_path_to_spent_value
            .iter()
            .map(|(block_path, value)| {
                let &BlockPath {
                    date_index,
                    block_index,
                } = block_path;

                let date_data = date_data_vec.get(date_index as usize).unwrap();

                (date_data, date_index, block_index, value)
            })
            .filter(|(date_data, date_index, _, _)| {
                let diff = len - 1 - (*date_index as usize);

                match self.filter {
                    UTXOFilter::Full => true,
                    UTXOFilter::From(from) => from <= diff,
                    UTXOFilter::To(to) => to > diff,
                    UTXOFilter::FromTo { from, to } => from <= diff && to > diff,
                    UTXOFilter::Year(year) => year == date_data.date.year() as usize,
                }
            })
            .map(|(date_data, _, block_index, value)| {
                let BlockData {
                    price: previous_price,
                    ..
                } = date_data.blocks.get(block_index as usize).unwrap();

                (previous_price, value)
            })
            .for_each(|(previous_price, value)| {
                let previous_dollar_amount = *previous_price as f64 * sats_to_btc(*value);
                let current_dollar_amount = price as f64 * sats_to_btc(*value);

                if previous_dollar_amount < current_dollar_amount {
                    realized_profit += (current_dollar_amount - previous_dollar_amount) as f32
                } else if current_dollar_amount < previous_dollar_amount {
                    realized_loss += (previous_dollar_amount - current_dollar_amount) as f32
                }
            });

        let mut total_supply = 0;
        let mut utxo_count = 0;

        let vec = {
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
            total_supply += block_data.amount;
            utxo_count += block_data.outputs_len as usize;

            (OrderedFloat(block_data.price), block_data.amount)
        })
        .sorted_unstable_by(|a, b| Ord::cmp(&a.0, &b.0))
        .collect_vec();

        self.prices.insert(
            PricesSubDatasetInsertData {
                height,
                price,
                realized_loss,
                realized_profit,
                total_supply,
                utxo_count,
            },
            #[allow(clippy::map_identity)]
            vec.iter().map(|(price, amount)| (price, amount)),
        )
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.prices.to_vec()
    }
}
