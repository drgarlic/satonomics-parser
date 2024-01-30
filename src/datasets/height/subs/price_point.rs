use std::fs;

use ordered_float::OrderedFloat;

use crate::{
    bitcoin::sats_to_btc,
    structs::{AnyHeightMap, HeightMap},
};

pub struct PricePointDataset {
    pp_mean: HeightMap<f32>,
    pp_median: HeightMap<f32>,
    pp_95p: HeightMap<f32>,
    pp_90p: HeightMap<f32>,
    pp_85p: HeightMap<f32>,
    pp_80p: HeightMap<f32>,
    pp_75p: HeightMap<f32>,
    pp_70p: HeightMap<f32>,
    pp_65p: HeightMap<f32>,
    pp_60p: HeightMap<f32>,
    pp_55p: HeightMap<f32>,
    pp_45p: HeightMap<f32>,
    pp_40p: HeightMap<f32>,
    pp_35p: HeightMap<f32>,
    pp_30p: HeightMap<f32>,
    pp_25p: HeightMap<f32>,
    pp_20p: HeightMap<f32>,
    pp_15p: HeightMap<f32>,
    pp_10p: HeightMap<f32>,
    pp_05p: HeightMap<f32>,
}

impl PricePointDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/price_point");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            pp_mean: HeightMap::new_on_disk_bin(&f("mean")),
            pp_median: HeightMap::new_on_disk_bin(&f("median")),
            pp_95p: HeightMap::new_on_disk_bin(&f("95p")),
            pp_90p: HeightMap::new_on_disk_bin(&f("90p")),
            pp_85p: HeightMap::new_on_disk_bin(&f("85p")),
            pp_80p: HeightMap::new_on_disk_bin(&f("80p")),
            pp_75p: HeightMap::new_on_disk_bin(&f("75p")),
            pp_70p: HeightMap::new_on_disk_bin(&f("70p")),
            pp_65p: HeightMap::new_on_disk_bin(&f("65p")),
            pp_60p: HeightMap::new_on_disk_bin(&f("60p")),
            pp_55p: HeightMap::new_on_disk_bin(&f("55p")),
            pp_45p: HeightMap::new_on_disk_bin(&f("45p")),
            pp_40p: HeightMap::new_on_disk_bin(&f("40p")),
            pp_35p: HeightMap::new_on_disk_bin(&f("35p")),
            pp_30p: HeightMap::new_on_disk_bin(&f("30p")),
            pp_25p: HeightMap::new_on_disk_bin(&f("25p")),
            pp_20p: HeightMap::new_on_disk_bin(&f("20p")),
            pp_15p: HeightMap::new_on_disk_bin(&f("15p")),
            pp_10p: HeightMap::new_on_disk_bin(&f("10p")),
            pp_05p: HeightMap::new_on_disk_bin(&f("05p")),
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

        let mean_price = {
            if total_supply_in_btc != 0.0 {
                (undivided_price_mean / total_supply_in_btc) as f32
            } else {
                0.0
            }
        };

        self.pp_mean.insert(height, mean_price);

        self.pp_05p.insert(height, price_05p.unwrap());
        self.pp_10p.insert(height, price_10p.unwrap());
        self.pp_15p.insert(height, price_15p.unwrap());
        self.pp_20p.insert(height, price_20p.unwrap());
        self.pp_25p.insert(height, price_25p.unwrap());
        self.pp_30p.insert(height, price_30p.unwrap());
        self.pp_35p.insert(height, price_35p.unwrap());
        self.pp_40p.insert(height, price_40p.unwrap());
        self.pp_45p.insert(height, price_45p.unwrap());
        self.pp_median.insert(height, price_median.unwrap());
        self.pp_55p.insert(height, price_55p.unwrap());
        self.pp_60p.insert(height, price_60p.unwrap());
        self.pp_65p.insert(height, price_65p.unwrap());
        self.pp_70p.insert(height, price_70p.unwrap());
        self.pp_75p.insert(height, price_75p.unwrap());
        self.pp_80p.insert(height, price_80p.unwrap());
        self.pp_85p.insert(height, price_85p.unwrap());
        self.pp_90p.insert(height, price_90p.unwrap());
        self.pp_95p.insert(height, price_95p.unwrap());
    }

    fn insert_default(&self, height: usize) {
        self.pp_mean.insert_default(height);

        self.pp_05p.insert_default(height);
        self.pp_10p.insert_default(height);
        self.pp_15p.insert_default(height);
        self.pp_20p.insert_default(height);
        self.pp_25p.insert_default(height);
        self.pp_30p.insert_default(height);
        self.pp_35p.insert_default(height);
        self.pp_40p.insert_default(height);
        self.pp_45p.insert_default(height);
        self.pp_median.insert_default(height);
        self.pp_55p.insert_default(height);
        self.pp_60p.insert_default(height);
        self.pp_65p.insert_default(height);
        self.pp_70p.insert_default(height);
        self.pp_75p.insert_default(height);
        self.pp_80p.insert_default(height);
        self.pp_85p.insert_default(height);
        self.pp_90p.insert_default(height);
        self.pp_95p.insert_default(height);
    }

    pub fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.pp_mean,
            &self.pp_95p,
            &self.pp_90p,
            &self.pp_85p,
            &self.pp_80p,
            &self.pp_75p,
            &self.pp_70p,
            &self.pp_65p,
            &self.pp_60p,
            &self.pp_55p,
            &self.pp_median,
            &self.pp_45p,
            &self.pp_40p,
            &self.pp_35p,
            &self.pp_30p,
            &self.pp_25p,
            &self.pp_20p,
            &self.pp_15p,
            &self.pp_10p,
            &self.pp_05p,
        ]
    }
}
