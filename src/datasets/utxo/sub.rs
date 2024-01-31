use chrono::Datelike;
use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::{
    bitcoin::sats_to_btc,
    datasets::{
        AnyDataset, PricePaidSubDataset, ProcessedBlockData, RealizedSubDataset, SupplySubDataset,
        UTXOsMetadataSubDataset, UnrealizedSubDataset,
    },
    structs::{AnyDateMap, BlockPath, WNaiveDate},
    structs::{AnyHeightMap, BlockData},
};

use super::UTXOFilter;

pub struct UTXOSubDataset {
    filter: UTXOFilter,
    price_paid: PricePaidSubDataset,
    realized: RealizedSubDataset,
    supply: SupplySubDataset,
    unrealized: UnrealizedSubDataset,
    utxos_metadata: UTXOsMetadataSubDataset,
}

impl UTXOSubDataset {
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

impl UTXOSubDataset {
    pub fn insert_block_data(
        &self,
        processed_block_data: &ProcessedBlockData,
        sorted_block_data_vec: Vec<(OrderedFloat<f32>, &WNaiveDate, &u64)>,
    ) {
        let &ProcessedBlockData {
            block_path_to_spent_value,
            block_price,
            is_date_last_block,
            states,
            ..
        } = processed_block_data;

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
                let current_dollar_amount = block_price as f64 * sats_to_btc(*value);

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
        .collect_vec();

        self.price_paid.insert(
            processed_block_data,
            total_supply,
            #[allow(clippy::map_identity)]
            vec.iter().map(|(price, amount)| (price, amount)),
        );

        self.realized
            .insert(processed_block_data, realized_loss, realized_profit);

        self.supply.insert(processed_block_data, total_supply);

        self.unrealized.insert_height(
            processed_block_data,
            #[allow(clippy::map_identity)]
            vec.iter().map(|(price, amount)| (price, amount)),
        );

        self.utxos_metadata.insert(processed_block_data, utxo_count);

        if is_date_last_block {
            self.unrealized.insert_date(
                processed_block_data,
                #[allow(clippy::map_identity)]
                vec.iter().map(|(price, amount)| (price, amount)),
            );
        }
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
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

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
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
