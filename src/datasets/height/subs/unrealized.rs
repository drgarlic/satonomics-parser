use std::fs;

use ordered_float::OrderedFloat;

use crate::{
    bitcoin::sats_to_btc,
    structs::{AnyHeightMap, HeightMap},
};

pub struct UnrealizedDataset {
    total_supply: HeightMap<u64>,
    supply_in_profit: HeightMap<u64>,

    unrealized_profit: HeightMap<f32>,
    unrealized_loss: HeightMap<f32>,

    price_mean: HeightMap<f32>,
    price_median: HeightMap<f32>,
    price_95p: HeightMap<f32>,
    price_90p: HeightMap<f32>,
    price_85p: HeightMap<f32>,
    price_80p: HeightMap<f32>,
    price_75p: HeightMap<f32>,
    price_70p: HeightMap<f32>,
    price_65p: HeightMap<f32>,
    price_60p: HeightMap<f32>,
    price_55p: HeightMap<f32>,
    price_45p: HeightMap<f32>,
    price_40p: HeightMap<f32>,
    price_35p: HeightMap<f32>,
    price_30p: HeightMap<f32>,
    price_25p: HeightMap<f32>,
    price_20p: HeightMap<f32>,
    price_15p: HeightMap<f32>,
    price_10p: HeightMap<f32>,
    price_05p: HeightMap<f32>,
}

impl UnrealizedDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let supply_path = format!("{parent_path}/supply");
        fs::create_dir_all(&supply_path)?;

        let unrealized_path = format!("{parent_path}/unrealized");
        fs::create_dir_all(&unrealized_path)?;

        let price_path = format!("{parent_path}/price");
        fs::create_dir_all(&price_path)?;

        let f1 = |s: &str| format!("{supply_path}/{s}");
        let f2 = |s: &str| format!("{unrealized_path}/{s}");
        let f3 = |s: &str| format!("{price_path}/{s}");

        Ok(Self {
            total_supply: HeightMap::new_on_disk_bin(&f1("total")),
            supply_in_profit: HeightMap::new_on_disk_bin(&f1("in_profit")),
            unrealized_profit: HeightMap::new_on_disk_bin(&f2("profit")),
            unrealized_loss: HeightMap::new_on_disk_bin(&f2("loss")),
            price_mean: HeightMap::new_on_disk_bin(&f3("mean")),
            price_median: HeightMap::new_on_disk_bin(&f3("median")),
            price_95p: HeightMap::new_on_disk_bin(&f3("95p")),
            price_90p: HeightMap::new_on_disk_bin(&f3("90p")),
            price_85p: HeightMap::new_on_disk_bin(&f3("85p")),
            price_80p: HeightMap::new_on_disk_bin(&f3("80p")),
            price_75p: HeightMap::new_on_disk_bin(&f3("75p")),
            price_70p: HeightMap::new_on_disk_bin(&f3("70p")),
            price_65p: HeightMap::new_on_disk_bin(&f3("65p")),
            price_60p: HeightMap::new_on_disk_bin(&f3("60p")),
            price_55p: HeightMap::new_on_disk_bin(&f3("55p")),
            price_45p: HeightMap::new_on_disk_bin(&f3("45p")),
            price_40p: HeightMap::new_on_disk_bin(&f3("40p")),
            price_35p: HeightMap::new_on_disk_bin(&f3("35p")),
            price_30p: HeightMap::new_on_disk_bin(&f3("30p")),
            price_25p: HeightMap::new_on_disk_bin(&f3("25p")),
            price_20p: HeightMap::new_on_disk_bin(&f3("20p")),
            price_15p: HeightMap::new_on_disk_bin(&f3("15p")),
            price_10p: HeightMap::new_on_disk_bin(&f3("10p")),
            price_05p: HeightMap::new_on_disk_bin(&f3("05p")),
        })
    }

    pub fn insert<'a>(
        &self,
        height: usize,
        block_price: f32,
        total_supply: u64,
        sorted_price_to_amount: impl Iterator<Item = (&'a OrderedFloat<f32>, &'a u64)>,
    ) {
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

        sorted_price_to_amount.for_each(|(price, sat_amount)| {
            let price = price.0;
            let sat_amount = *sat_amount;

            let btc_amount = sats_to_btc(sat_amount);

            if price < block_price {
                unrealized_profit += btc_amount * (block_price - price) as f64;
                supply_in_profit += sat_amount;
            } else if price > block_price {
                unrealized_loss += btc_amount * (price - block_price) as f64
            }

            undivided_price_mean += btc_amount * (price as f64);

            processed_amount_in_btc += btc_amount;

            if price_05p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.05 {
                price_05p.replace(price);
            }

            if price_10p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.1 {
                price_10p.replace(price);
            }

            if price_15p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.15 {
                price_15p.replace(price);
            }

            if price_20p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.2 {
                price_20p.replace(price);
            }

            if price_25p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.25 {
                price_25p.replace(price);
            }

            if price_30p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.3 {
                price_30p.replace(price);
            }

            if price_35p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.35 {
                price_35p.replace(price);
            }

            if price_40p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.4 {
                price_40p.replace(price);
            }

            if price_45p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.45 {
                price_45p.replace(price);
            }

            if price_median.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.5 {
                price_median.replace(price);
            }

            if price_55p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.55 {
                price_55p.replace(price);
            }

            if price_60p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.6 {
                price_60p.replace(price);
            }

            if price_65p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.65 {
                price_65p.replace(price);
            }

            if price_70p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.7 {
                price_70p.replace(price);
            }

            if price_75p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.75 {
                price_75p.replace(price);
            }

            if price_80p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.8 {
                price_80p.replace(price);
            }

            if price_85p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.85 {
                price_85p.replace(price);
            }

            if price_90p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.9 {
                price_90p.replace(price);
            }

            if price_95p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.95 {
                price_95p.replace(price);
            }
        });

        // Check if iter was empty
        if price_05p.is_none() {
            self.insert_default(height);
            return;
        }

        self.total_supply.insert(height, total_supply);

        self.supply_in_profit.insert(height, supply_in_profit);

        self.unrealized_profit
            .insert(height, unrealized_profit as f32);

        self.unrealized_loss.insert(height, unrealized_loss as f32);

        let mean_price = {
            if total_supply_in_btc != 0.0 {
                (undivided_price_mean / total_supply_in_btc) as f32
            } else {
                0.0
            }
        };

        self.price_mean.insert(height, mean_price);

        self.price_05p.insert(height, price_05p.unwrap());
        self.price_10p.insert(height, price_10p.unwrap());
        self.price_15p.insert(height, price_15p.unwrap());
        self.price_20p.insert(height, price_20p.unwrap());
        self.price_25p.insert(height, price_25p.unwrap());
        self.price_30p.insert(height, price_30p.unwrap());
        self.price_35p.insert(height, price_35p.unwrap());
        self.price_40p.insert(height, price_40p.unwrap());
        self.price_45p.insert(height, price_45p.unwrap());
        self.price_median.insert(height, price_median.unwrap());
        self.price_55p.insert(height, price_55p.unwrap());
        self.price_60p.insert(height, price_60p.unwrap());
        self.price_65p.insert(height, price_65p.unwrap());
        self.price_70p.insert(height, price_70p.unwrap());
        self.price_75p.insert(height, price_75p.unwrap());
        self.price_80p.insert(height, price_80p.unwrap());
        self.price_85p.insert(height, price_85p.unwrap());
        self.price_90p.insert(height, price_90p.unwrap());
        self.price_95p.insert(height, price_95p.unwrap());
    }

    pub fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.total_supply,
            &self.supply_in_profit,
            &self.unrealized_profit,
            &self.unrealized_loss,
            &self.price_mean,
            &self.price_median,
            &self.price_95p,
            &self.price_90p,
            &self.price_85p,
            &self.price_80p,
            &self.price_75p,
            &self.price_70p,
            &self.price_65p,
            &self.price_60p,
            &self.price_55p,
            &self.price_45p,
            &self.price_40p,
            &self.price_35p,
            &self.price_30p,
            &self.price_25p,
            &self.price_20p,
            &self.price_15p,
            &self.price_10p,
            &self.price_05p,
        ]
    }

    fn insert_default(&self, height: usize) {
        self.total_supply.insert_default(height);
        self.supply_in_profit.insert_default(height);

        self.unrealized_profit.insert_default(height);
        self.unrealized_loss.insert_default(height);

        self.price_mean.insert_default(height);

        self.price_05p.insert_default(height);
        self.price_10p.insert_default(height);
        self.price_15p.insert_default(height);
        self.price_20p.insert_default(height);
        self.price_25p.insert_default(height);
        self.price_30p.insert_default(height);
        self.price_35p.insert_default(height);
        self.price_40p.insert_default(height);
        self.price_45p.insert_default(height);
        self.price_median.insert_default(height);
        self.price_55p.insert_default(height);
        self.price_60p.insert_default(height);
        self.price_65p.insert_default(height);
        self.price_70p.insert_default(height);
        self.price_75p.insert_default(height);
        self.price_80p.insert_default(height);
        self.price_85p.insert_default(height);
        self.price_90p.insert_default(height);
        self.price_95p.insert_default(height);
    }
}
