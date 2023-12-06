use std::ops::ControlFlow;

use chrono::Datelike;
use itertools::Itertools;

use crate::structs::HeightMap;

use super::BlockDatasPerDay;

pub enum AgeRange {
    Full,
    To(usize),
    FromTo(usize, usize),
    From(usize),
    Year(usize),
}

pub struct HeightToAgedDataset {
    range: AgeRange,

    height_to_total_supply: HeightMap<f64>,
    height_to_supply_in_profit: HeightMap<f64>,
    // height_to_realized_profit: HeightMap<f32>,
    // height_to_realized_loss: HeightMap<f32>,
    height_to_unrealized_profit: HeightMap<f32>,
    height_to_unrealized_loss: HeightMap<f32>,
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

impl HeightToAgedDataset {
    pub fn import(name: &str, range: AgeRange) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("height_to_{}_{}.json", name, s);

        Ok(Self {
            range,
            height_to_total_supply: HeightMap::new(&f("total_supply")),
            height_to_supply_in_profit: HeightMap::new(&f("supply_in_profit")),
            // height_to_realized_profit: HeightMap::new(&f("realized_profit")),
            // height_to_realized_loss: HeightMap::new(&f("realized_loss")),
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

    pub fn get_min_last_height(&self) -> Option<usize> {
        [
            &self.height_to_total_supply.get_last_height(),
            &self.height_to_supply_in_profit.get_last_height(),
            &self.height_to_mean_price.get_last_height(),
            &self.height_to_median_price.get_last_height(),
            // &self.height_to_realized_profit.get_last_height(),
            // &self.height_to_realized_loss.get_last_height(),
            &self.height_to_unrealized_profit.get_last_height(),
            &self.height_to_unrealized_loss.get_last_height(),
            &self.height_to_95p_price.get_last_height(),
            &self.height_to_90p_price.get_last_height(),
            &self.height_to_85p_price.get_last_height(),
            &self.height_to_80p_price.get_last_height(),
            &self.height_to_75p_price.get_last_height(),
            &self.height_to_70p_price.get_last_height(),
            &self.height_to_65p_price.get_last_height(),
            &self.height_to_60p_price.get_last_height(),
            &self.height_to_55p_price.get_last_height(),
            &self.height_to_45p_price.get_last_height(),
            &self.height_to_40p_price.get_last_height(),
            &self.height_to_35p_price.get_last_height(),
            &self.height_to_30p_price.get_last_height(),
            &self.height_to_25p_price.get_last_height(),
            &self.height_to_20p_price.get_last_height(),
            &self.height_to_15p_price.get_last_height(),
            &self.height_to_10p_price.get_last_height(),
            &self.height_to_05p_price.get_last_height(),
            &self.height_to_utxo_count.get_last_height(),
        ]
        .iter()
        .min()
        .and_then(|opt| **opt)
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.height_to_total_supply.export()?;
        self.height_to_supply_in_profit.export()?;
        self.height_to_mean_price.export()?;
        self.height_to_median_price.export()?;
        self.height_to_unrealized_profit.export()?;
        self.height_to_unrealized_loss.export()?;
        self.height_to_95p_price.export()?;
        self.height_to_90p_price.export()?;
        self.height_to_85p_price.export()?;
        self.height_to_80p_price.export()?;
        self.height_to_75p_price.export()?;
        self.height_to_70p_price.export()?;
        self.height_to_65p_price.export()?;
        self.height_to_60p_price.export()?;
        self.height_to_55p_price.export()?;
        self.height_to_45p_price.export()?;
        self.height_to_40p_price.export()?;
        self.height_to_35p_price.export()?;
        self.height_to_30p_price.export()?;
        self.height_to_25p_price.export()?;
        self.height_to_20p_price.export()?;
        self.height_to_15p_price.export()?;
        self.height_to_10p_price.export()?;
        self.height_to_05p_price.export()?;
        self.height_to_utxo_count.export()?;

        Ok(())
    }

    pub fn insert(&self, block_datas_per_day: &BlockDatasPerDay, height: usize, price: f32) {
        let mut sliced_block_datas_per_day = {
            let len = block_datas_per_day.len();

            match self.range {
                AgeRange::Full => block_datas_per_day.iter().collect_vec(),
                AgeRange::From(from) => {
                    if from < len {
                        block_datas_per_day[..(len - from)].iter().collect_vec()
                    } else {
                        vec![]
                    }
                }
                AgeRange::To(to) => {
                    if to <= len {
                        block_datas_per_day[(len - to)..].iter().collect_vec()
                    } else {
                        block_datas_per_day.iter().collect_vec()
                    }
                }
                AgeRange::FromTo(from, to) => {
                    if from < len {
                        if to <= len {
                            block_datas_per_day[(len - to)..(len - from)]
                                .iter()
                                .collect_vec()
                        } else {
                            block_datas_per_day[..(len - from)].iter().collect_vec()
                        }
                    } else {
                        vec![]
                    }
                }
                AgeRange::Year(year) => block_datas_per_day
                    .iter()
                    .filter(|date_data| date_data.date.year() == year as i32)
                    .collect_vec(),
            }
        };

        if sliced_block_datas_per_day.is_empty() {
            self.height_to_utxo_count.insert(height, 0);

            self.height_to_total_supply.insert(height, 0.0);
            self.height_to_unrealized_profit.insert(height, 0.0);
            self.height_to_unrealized_loss.insert(height, 0.0);
            self.height_to_supply_in_profit.insert(height, 0.0);

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

        {
            let utxo_count = sliced_block_datas_per_day
                .iter()
                .map(|date_data| {
                    date_data
                        .blocks
                        .borrow()
                        .iter()
                        .map(|block_data| block_data.txid_index_to_outputs.borrow().len())
                        .sum::<usize>()
                })
                .sum();

            self.height_to_utxo_count.insert(height, utxo_count);
        }

        let mut amount_price_tuples = sliced_block_datas_per_day
            .drain(..)
            .flat_map(|date_data| {
                date_data
                    .blocks
                    .borrow()
                    .iter()
                    .map(|block_data| block_data.to_amount_price_tuple())
                    .collect_vec()
            })
            .collect_vec();

        amount_price_tuples.sort_by(|tuple_a, tuple_b| tuple_a.1.partial_cmp(&tuple_b.1).unwrap());

        let total_supply = amount_price_tuples
            .iter()
            .map(|(amount, _)| amount)
            .sum::<f64>();

        self.height_to_total_supply.insert(height, total_supply);

        {
            let tuples_in_profit = amount_price_tuples
                .iter()
                .filter(|(_, _price)| *_price < price)
                .collect_vec();

            let unrealized_profit = tuples_in_profit
                .iter()
                .map(|(amount, _price)| amount * (price - *_price) as f64)
                .sum::<f64>();

            self.height_to_unrealized_profit
                .insert(height, unrealized_profit as f32);

            let supply_in_profit = tuples_in_profit
                .iter()
                .map(|(amount, _)| amount)
                .sum::<f64>();

            self.height_to_supply_in_profit
                .insert(height, supply_in_profit);
        }

        {
            let tuples_in_loss = amount_price_tuples
                .iter()
                .filter(|(_, _price)| *_price > price)
                .collect_vec();

            let unrealized_loss = tuples_in_loss
                .iter()
                .map(|(amount, _price)| amount * (*_price - price) as f64)
                .sum::<f64>();

            self.height_to_unrealized_loss
                .insert(height, unrealized_loss as f32);
        }

        {
            let price_mean = (amount_price_tuples
                .iter()
                .map(|(amount, price)| amount * (*price as f64))
                .sum::<f64>()
                / total_supply) as f32;

            self.height_to_mean_price.insert(height, price_mean);
        }

        {
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

            let mut processed_amount = 0.0;

            amount_price_tuples.iter().try_for_each(|(amount, price)| {
                processed_amount += amount;

                if price_05p.is_none() && processed_amount >= total_supply * 0.05 {
                    price_05p.replace(price.to_owned());
                }

                if price_10p.is_none() && processed_amount >= total_supply * 0.1 {
                    price_10p.replace(price.to_owned());
                }

                if price_15p.is_none() && processed_amount >= total_supply * 0.15 {
                    price_15p.replace(price.to_owned());
                }

                if price_20p.is_none() && processed_amount >= total_supply * 0.2 {
                    price_20p.replace(price.to_owned());
                }

                if price_25p.is_none() && processed_amount >= total_supply * 0.25 {
                    price_25p.replace(price.to_owned());
                }

                if price_30p.is_none() && processed_amount >= total_supply * 0.3 {
                    price_30p.replace(price.to_owned());
                }

                if price_35p.is_none() && processed_amount >= total_supply * 0.35 {
                    price_35p.replace(price.to_owned());
                }

                if price_40p.is_none() && processed_amount >= total_supply * 0.4 {
                    price_40p.replace(price.to_owned());
                }

                if price_45p.is_none() && processed_amount >= total_supply * 0.45 {
                    price_45p.replace(price.to_owned());
                }

                if price_median.is_none() && processed_amount >= total_supply * 0.5 {
                    price_median.replace(price.to_owned());
                }

                if price_55p.is_none() && processed_amount >= total_supply * 0.55 {
                    price_55p.replace(price.to_owned());
                }

                if price_60p.is_none() && processed_amount >= total_supply * 0.6 {
                    price_60p.replace(price.to_owned());
                }

                if price_65p.is_none() && processed_amount >= total_supply * 0.65 {
                    price_65p.replace(price.to_owned());
                }

                if price_70p.is_none() && processed_amount >= total_supply * 0.7 {
                    price_70p.replace(price.to_owned());
                }

                if price_75p.is_none() && processed_amount >= total_supply * 0.75 {
                    price_75p.replace(price.to_owned());
                }

                if price_80p.is_none() && processed_amount >= total_supply * 0.8 {
                    price_80p.replace(price.to_owned());
                }

                if price_85p.is_none() && processed_amount >= total_supply * 0.85 {
                    price_85p.replace(price.to_owned());
                }

                if price_90p.is_none() && processed_amount >= total_supply * 0.9 {
                    price_90p.replace(price.to_owned());
                }

                if price_95p.is_none() && processed_amount >= total_supply * 0.95 {
                    price_95p.replace(price.to_owned());

                    return ControlFlow::Break(());
                }

                ControlFlow::Continue(())
            });

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
    }
}
