use chrono::Datelike;
use itertools::Itertools;

use crate::{
    bitcoin::sats_to_btc,
    computers::utxo_based::{structs::BlockPath, BlockData},
    structs::{AnyHeightMap, HeightMap},
};

use super::{HeightDataset, ProcessedData};

pub enum AgeRange {
    Full,
    To(usize),
    FromTo(usize, usize),
    From(usize),
    Year(usize),
}

pub struct AgedDataset {
    range: AgeRange,

    height_to_total_supply: HeightMap<u64>,
    height_to_supply_in_profit: HeightMap<u64>,
    height_to_unrealized_profit: HeightMap<f32>,
    height_to_unrealized_loss: HeightMap<f32>,
    /// NOTE: Fees not taken into account
    height_to_realized_profit: HeightMap<f32>,
    /// NOTE: Fees not taken into account
    height_to_realized_loss: HeightMap<f32>,
    height_to_mean_price: HeightMap<f32>,
    height_to_median_price: HeightMap<f32>,
    height_to_95p_price: HeightMap<f32>,
    height_to_90p_price: HeightMap<f32>,
    height_to_85p_price: HeightMap<f32>,
    height_to_80p_price: HeightMap<f32>,
    height_to_75p_price: HeightMap<f32>,
    height_to_70p_price: HeightMap<f32>,
    height_to_65p_price: HeightMap<f32>,
    height_to_60p_price: HeightMap<f32>,
    height_to_55p_price: HeightMap<f32>,
    height_to_45p_price: HeightMap<f32>,
    height_to_40p_price: HeightMap<f32>,
    height_to_35p_price: HeightMap<f32>,
    height_to_30p_price: HeightMap<f32>,
    height_to_25p_price: HeightMap<f32>,
    height_to_20p_price: HeightMap<f32>,
    height_to_15p_price: HeightMap<f32>,
    height_to_10p_price: HeightMap<f32>,
    height_to_05p_price: HeightMap<f32>,
    height_to_utxo_count: HeightMap<usize>,
}

impl AgedDataset {
    pub fn new(name: &str, range: AgeRange) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("height_to_{}_{}.json", name, s);

        Ok(Self {
            range,
            height_to_total_supply: HeightMap::new(&f("total_supply")),
            height_to_supply_in_profit: HeightMap::new(&f("supply_in_profit")),
            height_to_realized_profit: HeightMap::new(&f("realized_profit")),
            height_to_realized_loss: HeightMap::new(&f("realized_loss")),
            height_to_unrealized_profit: HeightMap::new(&f("unrealized_profit")),
            height_to_unrealized_loss: HeightMap::new(&f("unrealized_loss")),
            height_to_mean_price: HeightMap::new(&f("mean_price")),
            height_to_median_price: HeightMap::new(&f("median_price")),
            height_to_95p_price: HeightMap::new(&f("95p_price")),
            height_to_90p_price: HeightMap::new(&f("90p_price")),
            height_to_85p_price: HeightMap::new(&f("85p_price")),
            height_to_80p_price: HeightMap::new(&f("80p_price")),
            height_to_75p_price: HeightMap::new(&f("75p_price")),
            height_to_70p_price: HeightMap::new(&f("70p_price")),
            height_to_65p_price: HeightMap::new(&f("65p_price")),
            height_to_60p_price: HeightMap::new(&f("60p_price")),
            height_to_55p_price: HeightMap::new(&f("55p_price")),
            height_to_45p_price: HeightMap::new(&f("45p_price")),
            height_to_40p_price: HeightMap::new(&f("40p_price")),
            height_to_35p_price: HeightMap::new(&f("35p_price")),
            height_to_30p_price: HeightMap::new(&f("30p_price")),
            height_to_25p_price: HeightMap::new(&f("25p_price")),
            height_to_20p_price: HeightMap::new(&f("20p_price")),
            height_to_15p_price: HeightMap::new(&f("15p_price")),
            height_to_10p_price: HeightMap::new(&f("10p_price")),
            height_to_05p_price: HeightMap::new(&f("05p_price")),
            height_to_utxo_count: HeightMap::new(&f("utxo_count")),
        })
    }
}

impl<'a> HeightDataset<ProcessedData<'a>> for AgedDataset {
    fn insert(&self, processed_data: &ProcessedData) {
        let &ProcessedData {
            date_data_vec,
            price,
            height,
            block_path_to_spent_value,
            ..
        } = processed_data;

        let len = date_data_vec.len();

        let mut realized_profit = 0.0;
        let mut realized_loss = 0.0;

        let iter = block_path_to_spent_value
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

                match self.range {
                    AgeRange::Full => true,
                    AgeRange::From(from) => from <= diff,
                    AgeRange::To(to) => to > diff,
                    AgeRange::FromTo(from, to) => from <= diff && to > diff,
                    AgeRange::Year(year) => year == date_data.date.year() as usize,
                }
            })
            .map(|(date_data, _, block_index, value)| {
                let BlockData {
                    price: previous_price,
                    ..
                } = date_data.blocks.get(block_index as usize).unwrap();

                (previous_price, value)
            });

        iter.for_each(|(previous_price, value)| {
            let previous_dollar_amount = *previous_price as f64 * sats_to_btc(*value);
            let current_dollar_amount = price as f64 * sats_to_btc(*value);

            if previous_dollar_amount < current_dollar_amount {
                realized_profit += (current_dollar_amount - previous_dollar_amount) as f32
            } else if current_dollar_amount < previous_dollar_amount {
                realized_loss += (previous_dollar_amount - current_dollar_amount) as f32
            }
        });

        let sliced_date_data_vec = {
            match self.range {
                AgeRange::Full => date_data_vec.iter().collect_vec(),
                AgeRange::From(from) => {
                    if from < len {
                        date_data_vec[..(len - from)].iter().collect_vec()
                    } else {
                        vec![]
                    }
                }
                AgeRange::To(to) => {
                    if to <= len {
                        date_data_vec[(len - to)..].iter().collect_vec()
                    } else {
                        date_data_vec.iter().collect_vec()
                    }
                }
                AgeRange::FromTo(from, to) => {
                    if from < len {
                        if to <= len {
                            date_data_vec[(len - to)..(len - from)].iter().collect_vec()
                        } else {
                            date_data_vec[..(len - from)].iter().collect_vec()
                        }
                    } else {
                        vec![]
                    }
                }
                AgeRange::Year(year) => date_data_vec
                    .iter()
                    .filter(|date_data| date_data.date.year() == year as i32)
                    .collect_vec(),
            }
        };

        if sliced_date_data_vec.is_empty() {
            self.height_to_utxo_count.insert(height, 0);

            self.height_to_total_supply.insert(height, 0);
            self.height_to_unrealized_profit.insert(height, 0.0);
            self.height_to_unrealized_loss.insert(height, 0.0);
            self.height_to_supply_in_profit.insert(height, 0);

            self.height_to_realized_profit.insert(height, 0.0);
            self.height_to_realized_loss.insert(height, 0.0);

            self.height_to_mean_price.insert(height, 0.0);

            self.height_to_05p_price.insert(height, 0.0);
            self.height_to_10p_price.insert(height, 0.0);
            self.height_to_15p_price.insert(height, 0.0);
            self.height_to_20p_price.insert(height, 0.0);
            self.height_to_25p_price.insert(height, 0.0);
            self.height_to_30p_price.insert(height, 0.0);
            self.height_to_35p_price.insert(height, 0.0);
            self.height_to_40p_price.insert(height, 0.0);
            self.height_to_45p_price.insert(height, 0.0);
            self.height_to_median_price.insert(height, 0.0);
            self.height_to_55p_price.insert(height, 0.0);
            self.height_to_60p_price.insert(height, 0.0);
            self.height_to_65p_price.insert(height, 0.0);
            self.height_to_70p_price.insert(height, 0.0);
            self.height_to_75p_price.insert(height, 0.0);
            self.height_to_80p_price.insert(height, 0.0);
            self.height_to_85p_price.insert(height, 0.0);
            self.height_to_90p_price.insert(height, 0.0);
            self.height_to_95p_price.insert(height, 0.0);

            return;
        }

        let mut utxo_count = 0;

        let mut block_data_vec = sliced_date_data_vec
            .iter()
            .flat_map(|date_data| {
                utxo_count += date_data
                    .blocks
                    .iter()
                    .map(|block| block.outputs_len as usize)
                    .sum::<usize>();

                &date_data.blocks
            })
            .collect_vec();

        block_data_vec.sort_unstable_by(|tuple_a, tuple_b| {
            tuple_a.price.partial_cmp(&tuple_b.price).unwrap()
        });

        let total_supply = block_data_vec
            .iter()
            .map(|block_data| block_data.amount)
            .sum();

        let mut unrealized_profit = 0.0;
        let mut unrealized_loss = 0.0;

        let mut supply_in_profit = 0;

        let mut undivided_price_mean = 0.0;

        let mut price_05p = None;
        let mut price_10p = None;
        let mut price_15p = None;
        let mut price_20p = None;
        let mut price_25p = None;
        let mut price_30p = None;
        let mut price_35p = None;
        let mut price_40p = None;
        let mut price_45p = None;
        let mut price_median = None;
        let mut price_55p = None;
        let mut price_60p = None;
        let mut price_65p = None;
        let mut price_70p = None;
        let mut price_75p = None;
        let mut price_80p = None;
        let mut price_85p = None;
        let mut price_90p = None;
        let mut price_95p = None;

        let mut processed_amount_in_btc = 0.0;

        let total_supply_in_btc = sats_to_btc(total_supply);

        block_data_vec.iter().for_each(|block_data| {
            processed_amount_in_btc += sats_to_btc(block_data.amount);

            if block_data.price < price {
                unrealized_profit +=
                    sats_to_btc(block_data.amount) * (price - block_data.price) as f64;
                supply_in_profit += block_data.amount;
            } else if block_data.price > price {
                unrealized_loss +=
                    sats_to_btc(block_data.amount) * (block_data.price - price) as f64
            }

            undivided_price_mean += sats_to_btc(block_data.amount) * (block_data.price as f64);

            if price_05p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.05 {
                price_05p.replace(block_data.price);
            }

            if price_10p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.1 {
                price_10p.replace(block_data.price);
            }

            if price_15p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.15 {
                price_15p.replace(block_data.price);
            }

            if price_20p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.2 {
                price_20p.replace(block_data.price);
            }

            if price_25p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.25 {
                price_25p.replace(block_data.price);
            }

            if price_30p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.3 {
                price_30p.replace(block_data.price);
            }

            if price_35p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.35 {
                price_35p.replace(block_data.price);
            }

            if price_40p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.4 {
                price_40p.replace(block_data.price);
            }

            if price_45p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.45 {
                price_45p.replace(block_data.price);
            }

            if price_median.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.5 {
                price_median.replace(block_data.price);
            }

            if price_55p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.55 {
                price_55p.replace(block_data.price);
            }

            if price_60p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.6 {
                price_60p.replace(block_data.price);
            }

            if price_65p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.65 {
                price_65p.replace(block_data.price);
            }

            if price_70p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.7 {
                price_70p.replace(block_data.price);
            }

            if price_75p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.75 {
                price_75p.replace(block_data.price);
            }

            if price_80p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.8 {
                price_80p.replace(block_data.price);
            }

            if price_85p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.85 {
                price_85p.replace(block_data.price);
            }

            if price_90p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.9 {
                price_90p.replace(block_data.price);
            }

            if price_95p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.95 {
                price_95p.replace(block_data.price);
            }
        });

        self.height_to_utxo_count.insert(height, utxo_count);

        self.height_to_total_supply.insert(height, total_supply);

        self.height_to_supply_in_profit
            .insert(height, supply_in_profit);

        self.height_to_unrealized_profit
            .insert(height, unrealized_profit as f32);

        self.height_to_unrealized_loss
            .insert(height, unrealized_loss as f32);

        self.height_to_realized_profit
            .insert(height, realized_profit);

        self.height_to_realized_loss.insert(height, realized_loss);

        self.height_to_mean_price
            .insert(height, (undivided_price_mean / total_supply_in_btc) as f32);

        if let Some(price) = price_05p {
            self.height_to_05p_price.insert(height, price)
        }

        if let Some(price) = price_10p {
            self.height_to_10p_price.insert(height, price)
        }

        if let Some(price) = price_15p {
            self.height_to_15p_price.insert(height, price)
        }

        if let Some(price) = price_20p {
            self.height_to_20p_price.insert(height, price)
        }

        if let Some(price) = price_25p {
            self.height_to_25p_price.insert(height, price)
        }

        if let Some(price) = price_30p {
            self.height_to_30p_price.insert(height, price)
        }

        if let Some(price) = price_35p {
            self.height_to_35p_price.insert(height, price)
        }

        if let Some(price) = price_40p {
            self.height_to_40p_price.insert(height, price)
        }

        if let Some(price) = price_45p {
            self.height_to_45p_price.insert(height, price)
        }

        if let Some(price) = price_median {
            self.height_to_median_price.insert(height, price)
        }

        if let Some(price) = price_55p {
            self.height_to_55p_price.insert(height, price)
        }

        if let Some(price) = price_60p {
            self.height_to_60p_price.insert(height, price)
        }

        if let Some(price) = price_65p {
            self.height_to_65p_price.insert(height, price)
        }

        if let Some(price) = price_70p {
            self.height_to_70p_price.insert(height, price)
        }

        if let Some(price) = price_75p {
            self.height_to_75p_price.insert(height, price)
        }

        if let Some(price) = price_80p {
            self.height_to_80p_price.insert(height, price)
        }

        if let Some(price) = price_85p {
            self.height_to_85p_price.insert(height, price)
        }

        if let Some(price) = price_90p {
            self.height_to_90p_price.insert(height, price)
        }

        if let Some(price) = price_95p {
            self.height_to_95p_price.insert(height, price)
        }
    }

    fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.height_to_total_supply,
            &self.height_to_supply_in_profit,
            &self.height_to_mean_price,
            &self.height_to_median_price,
            &self.height_to_realized_profit,
            &self.height_to_realized_loss,
            &self.height_to_unrealized_profit,
            &self.height_to_unrealized_loss,
            &self.height_to_95p_price,
            &self.height_to_90p_price,
            &self.height_to_85p_price,
            &self.height_to_80p_price,
            &self.height_to_75p_price,
            &self.height_to_70p_price,
            &self.height_to_65p_price,
            &self.height_to_60p_price,
            &self.height_to_55p_price,
            &self.height_to_45p_price,
            &self.height_to_40p_price,
            &self.height_to_35p_price,
            &self.height_to_30p_price,
            &self.height_to_25p_price,
            &self.height_to_20p_price,
            &self.height_to_15p_price,
            &self.height_to_10p_price,
            &self.height_to_05p_price,
            &self.height_to_utxo_count,
        ]
    }
}
