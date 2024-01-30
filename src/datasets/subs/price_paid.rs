use chrono::NaiveDate;
use ordered_float::OrderedFloat;

use crate::{
    bitcoin::sats_to_btc,
    datasets::ProcessedBlockData,
    structs::{AnyDateMap, AnyHeightMap, BiMap},
};

pub struct PricePaidSubDataset {
    pp_mean: BiMap<f32>,
    pp_median: BiMap<f32>,
    pp_95p: BiMap<f32>,
    pp_90p: BiMap<f32>,
    pp_85p: BiMap<f32>,
    pp_80p: BiMap<f32>,
    pp_75p: BiMap<f32>,
    pp_70p: BiMap<f32>,
    pp_65p: BiMap<f32>,
    pp_60p: BiMap<f32>,
    pp_55p: BiMap<f32>,
    pp_45p: BiMap<f32>,
    pp_40p: BiMap<f32>,
    pp_35p: BiMap<f32>,
    pp_30p: BiMap<f32>,
    pp_25p: BiMap<f32>,
    pp_20p: BiMap<f32>,
    pp_15p: BiMap<f32>,
    pp_10p: BiMap<f32>,
    pp_05p: BiMap<f32>,
}

impl PricePaidSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/price_paid");
        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            pp_mean: BiMap::new_on_disk_bin(&f("mean")),
            pp_median: BiMap::new_on_disk_bin(&f("median")),
            pp_95p: BiMap::new_on_disk_bin(&f("95p")),
            pp_90p: BiMap::new_on_disk_bin(&f("90p")),
            pp_85p: BiMap::new_on_disk_bin(&f("85p")),
            pp_80p: BiMap::new_on_disk_bin(&f("80p")),
            pp_75p: BiMap::new_on_disk_bin(&f("75p")),
            pp_70p: BiMap::new_on_disk_bin(&f("70p")),
            pp_65p: BiMap::new_on_disk_bin(&f("65p")),
            pp_60p: BiMap::new_on_disk_bin(&f("60p")),
            pp_55p: BiMap::new_on_disk_bin(&f("55p")),
            pp_45p: BiMap::new_on_disk_bin(&f("45p")),
            pp_40p: BiMap::new_on_disk_bin(&f("40p")),
            pp_35p: BiMap::new_on_disk_bin(&f("35p")),
            pp_30p: BiMap::new_on_disk_bin(&f("30p")),
            pp_25p: BiMap::new_on_disk_bin(&f("25p")),
            pp_20p: BiMap::new_on_disk_bin(&f("20p")),
            pp_15p: BiMap::new_on_disk_bin(&f("15p")),
            pp_10p: BiMap::new_on_disk_bin(&f("10p")),
            pp_05p: BiMap::new_on_disk_bin(&f("05p")),
        })
    }

    pub fn insert<'a>(
        &self,
        &ProcessedBlockData {
            date,
            height,
            block_price,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
        total_supply: u64,
        sorted_price_to_amount: impl Iterator<Item = (&'a OrderedFloat<f32>, &'a u64)>,
    ) {
        let mut unrealized_profit = 0.0;
        let mut unrealized_loss = 0.0;

        let mut supply_in_profit = 0;

        let mut undivided_price_mean = 0.0;

        let mut pp_05p = None;
        let mut pp_10p = None;
        let mut pp_15p = None;
        let mut pp_20p = None;
        let mut pp_25p = None;
        let mut pp_30p = None;
        let mut pp_35p = None;
        let mut pp_40p = None;
        let mut pp_45p = None;
        let mut pp_median = None;
        let mut pp_55p = None;
        let mut pp_60p = None;
        let mut pp_65p = None;
        let mut pp_70p = None;
        let mut pp_75p = None;
        let mut pp_80p = None;
        let mut pp_85p = None;
        let mut pp_90p = None;
        let mut pp_95p = None;

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

            if pp_05p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.05 {
                pp_05p.replace(price);
            }

            if pp_10p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.1 {
                pp_10p.replace(price);
            }

            if pp_15p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.15 {
                pp_15p.replace(price);
            }

            if pp_20p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.2 {
                pp_20p.replace(price);
            }

            if pp_25p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.25 {
                pp_25p.replace(price);
            }

            if pp_30p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.3 {
                pp_30p.replace(price);
            }

            if pp_35p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.35 {
                pp_35p.replace(price);
            }

            if pp_40p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.4 {
                pp_40p.replace(price);
            }

            if pp_45p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.45 {
                pp_45p.replace(price);
            }

            if pp_median.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.5 {
                pp_median.replace(price);
            }

            if pp_55p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.55 {
                pp_55p.replace(price);
            }

            if pp_60p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.6 {
                pp_60p.replace(price);
            }

            if pp_65p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.65 {
                pp_65p.replace(price);
            }

            if pp_70p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.7 {
                pp_70p.replace(price);
            }

            if pp_75p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.75 {
                pp_75p.replace(price);
            }

            if pp_80p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.8 {
                pp_80p.replace(price);
            }

            if pp_85p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.85 {
                pp_85p.replace(price);
            }

            if pp_90p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.9 {
                pp_90p.replace(price);
            }

            if pp_95p.is_none() && processed_amount_in_btc >= total_supply_in_btc * 0.95 {
                pp_95p.replace(price);
            }
        });

        // Check if iter was empty
        if pp_05p.is_none() {
            self.insert_height_default(height);

            if is_date_last_block {
                self.insert_date_default(date);
            }

            return;
        }

        let mean_price = {
            if total_supply_in_btc != 0.0 {
                (undivided_price_mean / total_supply_in_btc) as f32
            } else {
                0.0
            }
        };

        self.pp_mean.height.insert(height, mean_price);
        self.pp_05p.height.insert(height, pp_05p.unwrap());
        self.pp_10p.height.insert(height, pp_10p.unwrap());
        self.pp_15p.height.insert(height, pp_15p.unwrap());
        self.pp_20p.height.insert(height, pp_20p.unwrap());
        self.pp_25p.height.insert(height, pp_25p.unwrap());
        self.pp_30p.height.insert(height, pp_30p.unwrap());
        self.pp_35p.height.insert(height, pp_35p.unwrap());
        self.pp_40p.height.insert(height, pp_40p.unwrap());
        self.pp_45p.height.insert(height, pp_45p.unwrap());
        self.pp_median.height.insert(height, pp_median.unwrap());
        self.pp_55p.height.insert(height, pp_55p.unwrap());
        self.pp_60p.height.insert(height, pp_60p.unwrap());
        self.pp_65p.height.insert(height, pp_65p.unwrap());
        self.pp_70p.height.insert(height, pp_70p.unwrap());
        self.pp_75p.height.insert(height, pp_75p.unwrap());
        self.pp_80p.height.insert(height, pp_80p.unwrap());
        self.pp_85p.height.insert(height, pp_85p.unwrap());
        self.pp_90p.height.insert(height, pp_90p.unwrap());
        self.pp_95p.height.insert(height, pp_95p.unwrap());

        if is_date_last_block {
            self.pp_mean.date.insert(date, mean_price);
            self.pp_05p.date.insert(date, pp_05p.unwrap());
            self.pp_10p.date.insert(date, pp_10p.unwrap());
            self.pp_15p.date.insert(date, pp_15p.unwrap());
            self.pp_20p.date.insert(date, pp_20p.unwrap());
            self.pp_25p.date.insert(date, pp_25p.unwrap());
            self.pp_30p.date.insert(date, pp_30p.unwrap());
            self.pp_35p.date.insert(date, pp_35p.unwrap());
            self.pp_40p.date.insert(date, pp_40p.unwrap());
            self.pp_45p.date.insert(date, pp_45p.unwrap());
            self.pp_median.date.insert(date, pp_median.unwrap());
            self.pp_55p.date.insert(date, pp_55p.unwrap());
            self.pp_60p.date.insert(date, pp_60p.unwrap());
            self.pp_65p.date.insert(date, pp_65p.unwrap());
            self.pp_70p.date.insert(date, pp_70p.unwrap());
            self.pp_75p.date.insert(date, pp_75p.unwrap());
            self.pp_80p.date.insert(date, pp_80p.unwrap());
            self.pp_85p.date.insert(date, pp_85p.unwrap());
            self.pp_90p.date.insert(date, pp_90p.unwrap());
            self.pp_95p.date.insert(date, pp_95p.unwrap());
        }
    }

    fn insert_height_default(&self, height: usize) {
        self.pp_mean.height.insert_default(height);
        self.pp_05p.height.insert_default(height);
        self.pp_10p.height.insert_default(height);
        self.pp_15p.height.insert_default(height);
        self.pp_20p.height.insert_default(height);
        self.pp_25p.height.insert_default(height);
        self.pp_30p.height.insert_default(height);
        self.pp_35p.height.insert_default(height);
        self.pp_40p.height.insert_default(height);
        self.pp_45p.height.insert_default(height);
        self.pp_median.height.insert_default(height);
        self.pp_55p.height.insert_default(height);
        self.pp_60p.height.insert_default(height);
        self.pp_65p.height.insert_default(height);
        self.pp_70p.height.insert_default(height);
        self.pp_75p.height.insert_default(height);
        self.pp_80p.height.insert_default(height);
        self.pp_85p.height.insert_default(height);
        self.pp_90p.height.insert_default(height);
        self.pp_95p.height.insert_default(height);
    }

    fn insert_date_default(&self, date: NaiveDate) {
        self.pp_mean.date.insert_default(date);
        self.pp_05p.date.insert_default(date);
        self.pp_10p.date.insert_default(date);
        self.pp_15p.date.insert_default(date);
        self.pp_20p.date.insert_default(date);
        self.pp_25p.date.insert_default(date);
        self.pp_30p.date.insert_default(date);
        self.pp_35p.date.insert_default(date);
        self.pp_40p.date.insert_default(date);
        self.pp_45p.date.insert_default(date);
        self.pp_median.date.insert_default(date);
        self.pp_55p.date.insert_default(date);
        self.pp_60p.date.insert_default(date);
        self.pp_65p.date.insert_default(date);
        self.pp_70p.date.insert_default(date);
        self.pp_75p.date.insert_default(date);
        self.pp_80p.date.insert_default(date);
        self.pp_85p.date.insert_default(date);
        self.pp_90p.date.insert_default(date);
        self.pp_95p.date.insert_default(date);
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.pp_mean.height,
            &self.pp_95p.height,
            &self.pp_90p.height,
            &self.pp_85p.height,
            &self.pp_80p.height,
            &self.pp_75p.height,
            &self.pp_70p.height,
            &self.pp_65p.height,
            &self.pp_60p.height,
            &self.pp_55p.height,
            &self.pp_median.height,
            &self.pp_45p.height,
            &self.pp_40p.height,
            &self.pp_35p.height,
            &self.pp_30p.height,
            &self.pp_25p.height,
            &self.pp_20p.height,
            &self.pp_15p.height,
            &self.pp_10p.height,
            &self.pp_05p.height,
        ]
    }

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.pp_mean.date,
            &self.pp_95p.date,
            &self.pp_90p.date,
            &self.pp_85p.date,
            &self.pp_80p.date,
            &self.pp_75p.date,
            &self.pp_70p.date,
            &self.pp_65p.date,
            &self.pp_60p.date,
            &self.pp_55p.date,
            &self.pp_median.date,
            &self.pp_45p.date,
            &self.pp_40p.date,
            &self.pp_35p.date,
            &self.pp_30p.date,
            &self.pp_25p.date,
            &self.pp_20p.date,
            &self.pp_15p.date,
            &self.pp_10p.date,
            &self.pp_05p.date,
        ]
    }
}
