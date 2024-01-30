use ordered_float::OrderedFloat;

use crate::{
    bitcoin::sats_to_btc,
    datasets::ProcessedBlockData,
    structs::{AnyDateMap, AnyHeightMap, BiMap},
};

pub struct UnrealizedSubDataset {
    supply_in_profit: BiMap<u64>,
    unrealized_profit: BiMap<f32>,
    unrealized_loss: BiMap<f32>,
}

struct ComputedResult {
    supply_in_profit: u64,
    unrealized_profit: f32,
    unrealized_loss: f32,
}

impl UnrealizedSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let supply_path = format!("{parent_path}/supply");
        let unrealized_path = format!("{parent_path}/unrealized");
        let f1 = |s: &str| format!("{supply_path}/{s}");
        let f2 = |s: &str| format!("{unrealized_path}/{s}");

        Ok(Self {
            supply_in_profit: BiMap::new_on_disk_bin(&f1("in_profit")),
            unrealized_profit: BiMap::new_on_disk_bin(&f2("profit")),
            unrealized_loss: BiMap::new_on_disk_bin(&f2("loss")),
        })
    }

    pub fn insert_height<'a>(
        &self,
        &ProcessedBlockData {
            height,
            block_price: price,
            ..
        }: &ProcessedBlockData,
        sorted_price_to_amount: impl Iterator<Item = (&'a OrderedFloat<f32>, &'a u64)>,
    ) {
        let ComputedResult {
            supply_in_profit,
            unrealized_loss,
            unrealized_profit,
        } = self.compute(price, sorted_price_to_amount);

        self.supply_in_profit
            .height
            .insert(height, supply_in_profit);

        self.unrealized_profit
            .height
            .insert(height, unrealized_profit);

        self.unrealized_loss.height.insert(height, unrealized_loss);
    }

    pub fn insert_date<'a>(
        &self,
        &ProcessedBlockData {
            date,
            date_price: price,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
        sorted_price_to_amount: impl Iterator<Item = (&'a OrderedFloat<f32>, &'a u64)>,
    ) {
        if !is_date_last_block {
            unreachable!()
        }

        let ComputedResult {
            supply_in_profit,
            unrealized_loss,
            unrealized_profit,
        } = self.compute(price, sorted_price_to_amount);

        self.supply_in_profit.date.insert(date, supply_in_profit);

        self.unrealized_profit.date.insert(date, unrealized_profit);

        self.unrealized_loss.date.insert(date, unrealized_loss);
    }

    fn compute<'a>(
        &self,
        ref_price: f32,
        sorted_price_to_amount: impl Iterator<Item = (&'a OrderedFloat<f32>, &'a u64)>,
    ) -> ComputedResult {
        let mut unrealized_profit = 0.0;
        let mut unrealized_loss = 0.0;

        let mut supply_in_profit = 0;

        sorted_price_to_amount.for_each(|(price_acquired, sat_amount)| {
            let price = price_acquired.0;
            let sat_amount = *sat_amount;

            let btc_amount = sats_to_btc(sat_amount);

            if price < ref_price {
                unrealized_profit += btc_amount * (ref_price - price) as f64;
                supply_in_profit += sat_amount;
            } else if price > ref_price {
                unrealized_loss += btc_amount * (price - ref_price) as f64
            }
        });

        ComputedResult {
            supply_in_profit,
            unrealized_loss: unrealized_loss as f32,
            unrealized_profit: unrealized_profit as f32,
        }
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.supply_in_profit.height,
            &self.unrealized_profit.height,
            &self.unrealized_loss.height,
        ]
    }

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.supply_in_profit.date,
            &self.unrealized_profit.date,
            &self.unrealized_loss.date,
        ]
    }
}
