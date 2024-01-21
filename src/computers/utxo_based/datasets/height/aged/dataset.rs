use std::collections::BTreeMap;

use chrono::Datelike;
use ordered_float::OrderedFloat;

use crate::{
    bitcoin::sats_to_btc,
    computers::utxo_based::{
        datasets::height::{PriceDataset, PriceDatasetInsertData},
        structs::BlockPath,
        BlockData, HeightDatasetTrait, ProcessedData,
    },
    structs::AnyHeightMap,
};

use super::AgeFilter;

pub struct AgedDataset {
    filter: AgeFilter,
    price_dataset: PriceDataset,
}

impl AgedDataset {
    pub fn import(path: &str, name: &str, range: AgeFilter) -> color_eyre::Result<Self> {
        Ok(Self {
            filter: range,
            price_dataset: PriceDataset::import(path, name)?,
        })
    }
}

impl HeightDatasetTrait for AgedDataset {
    fn insert(&self, processed_data: &ProcessedData) {
        let &ProcessedData {
            states,
            price,
            height,
            block_path_to_spent_value,
            ..
        } = processed_data;

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
                    AgeFilter::Full => true,
                    AgeFilter::From(from) => from <= diff,
                    AgeFilter::To(to) => to > diff,
                    AgeFilter::FromTo { from, to } => from <= diff && to > diff,
                    AgeFilter::Year(year) => year == date_data.date.year() as usize,
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

        let price_to_amount = {
            match self.filter {
                AgeFilter::Full => date_data_vec.iter(),
                AgeFilter::From(from) if from < len => date_data_vec[..(len - from)].iter(),
                AgeFilter::To(to) => {
                    if to <= len {
                        date_data_vec[(len - to)..].iter()
                    } else {
                        date_data_vec.iter()
                    }
                }
                AgeFilter::FromTo { from, to } if from < len => {
                    if to <= len {
                        date_data_vec[(len - to)..(len - from)].iter()
                    } else {
                        date_data_vec[..(len - from)].iter()
                    }
                }
                AgeFilter::Year(_) => date_data_vec.iter(),
                _ => date_data_vec[..0].iter(),
            }
        }
        // Can't figure how to put the filter inside the match without type issues
        .filter(|date_data| {
            if let AgeFilter::Year(year) = self.filter {
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
        .collect::<BTreeMap<_, _>>();

        self.price_dataset.insert(PriceDatasetInsertData {
            height,
            price,
            price_to_amount,
            realized_loss,
            realized_profit,
            total_supply,
            utxo_count,
        })
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.price_dataset.to_vec()
    }
}
