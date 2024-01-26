use ordered_float::OrderedFloat;

use crate::{
    bitcoin::sats_to_btc,
    structs::{AnyHeightMap, HeightMap},
};

pub struct PricesSubDatasetInsertData {
    pub height: usize,
    pub price: f32,
    pub realized_loss: f32,
    pub realized_profit: f32,
    pub total_supply: u64,
    pub utxo_count: usize,
}

pub struct PricesSubDataset {
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

impl PricesSubDataset {
    pub fn import(folder_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{folder_path}/{s}.json");

        Ok(Self {
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

    pub fn insert<'a>(
        &self,
        PricesSubDatasetInsertData {
            height,
            price: block_price,
            realized_loss,
            realized_profit,
            total_supply,
            utxo_count,
            ..
        }: PricesSubDatasetInsertData,
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

        let mean_price = {
            if total_supply_in_btc != 0.0 {
                (undivided_price_mean / total_supply_in_btc) as f32
            } else {
                0.0
            }
        };

        self.height_to_mean_price.insert(height, mean_price);

        self.height_to_05p_price.insert(height, price_05p.unwrap());
        self.height_to_10p_price.insert(height, price_10p.unwrap());
        self.height_to_15p_price.insert(height, price_15p.unwrap());
        self.height_to_20p_price.insert(height, price_20p.unwrap());
        self.height_to_25p_price.insert(height, price_25p.unwrap());
        self.height_to_30p_price.insert(height, price_30p.unwrap());
        self.height_to_35p_price.insert(height, price_35p.unwrap());
        self.height_to_40p_price.insert(height, price_40p.unwrap());
        self.height_to_45p_price.insert(height, price_45p.unwrap());
        self.height_to_median_price
            .insert(height, price_median.unwrap());
        self.height_to_55p_price.insert(height, price_55p.unwrap());
        self.height_to_60p_price.insert(height, price_60p.unwrap());
        self.height_to_65p_price.insert(height, price_65p.unwrap());
        self.height_to_70p_price.insert(height, price_70p.unwrap());
        self.height_to_75p_price.insert(height, price_75p.unwrap());
        self.height_to_80p_price.insert(height, price_80p.unwrap());
        self.height_to_85p_price.insert(height, price_85p.unwrap());
        self.height_to_90p_price.insert(height, price_90p.unwrap());
        self.height_to_95p_price.insert(height, price_95p.unwrap());
    }

    pub fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
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

    fn insert_default(&self, height: usize) {
        self.height_to_utxo_count.insert_default(height);

        self.height_to_total_supply.insert_default(height);
        self.height_to_unrealized_profit.insert_default(height);
        self.height_to_unrealized_loss.insert_default(height);
        self.height_to_supply_in_profit.insert_default(height);

        self.height_to_realized_profit.insert_default(height);
        self.height_to_realized_loss.insert_default(height);

        self.height_to_mean_price.insert_default(height);

        self.height_to_05p_price.insert_default(height);
        self.height_to_10p_price.insert_default(height);
        self.height_to_15p_price.insert_default(height);
        self.height_to_20p_price.insert_default(height);
        self.height_to_25p_price.insert_default(height);
        self.height_to_30p_price.insert_default(height);
        self.height_to_35p_price.insert_default(height);
        self.height_to_40p_price.insert_default(height);
        self.height_to_45p_price.insert_default(height);
        self.height_to_median_price.insert_default(height);
        self.height_to_55p_price.insert_default(height);
        self.height_to_60p_price.insert_default(height);
        self.height_to_65p_price.insert_default(height);
        self.height_to_70p_price.insert_default(height);
        self.height_to_75p_price.insert_default(height);
        self.height_to_80p_price.insert_default(height);
        self.height_to_85p_price.insert_default(height);
        self.height_to_90p_price.insert_default(height);
        self.height_to_95p_price.insert_default(height);
    }
}
