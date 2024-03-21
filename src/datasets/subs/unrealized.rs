use std::ops::Add;

use chrono::NaiveDate;

use crate::{
    bitcoin::sats_to_btc,
    datasets::ProcessedBlockData,
    structs::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap},
};

pub struct UnrealizedSubDataset {
    supply_in_profit: BiMap<f64>,
    unrealized_profit: BiMap<f32>,
    unrealized_loss: BiMap<f32>,
}

#[derive(Debug, Default)]
pub struct UnrealizedState {
    supply_in_profit: u64,
    unrealized_profit: f64,
    unrealized_loss: f64,
}

impl UnrealizedState {
    #[inline]
    pub fn iterate(&mut self, price_then: f32, price_now: f32, sat_amount: u64, btc_amount: f64) {
        if price_then < price_now {
            self.unrealized_profit += btc_amount * ((price_now - price_then) as f64);
            self.supply_in_profit += sat_amount;
        } else if price_then > price_now {
            self.unrealized_loss += btc_amount * ((price_then - price_now) as f64)
        }
    }
}

impl Add<UnrealizedState> for UnrealizedState {
    type Output = UnrealizedState;

    fn add(self, other: UnrealizedState) -> UnrealizedState {
        UnrealizedState {
            supply_in_profit: self.supply_in_profit + other.supply_in_profit,
            unrealized_profit: self.unrealized_profit + other.unrealized_profit,
            unrealized_loss: self.unrealized_loss + other.unrealized_loss,
        }
    }
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

    pub fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool {
        self.to_vec()
            .iter()
            .all(|bi| bi.are_date_and_height_safe(date, height))
    }

    pub fn insert(
        &self,
        &ProcessedBlockData {
            height,
            date,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
        height_state: &UnrealizedState,
        date_state: &UnrealizedState,
    ) {
        self.supply_in_profit
            .height
            .insert(height, sats_to_btc(height_state.supply_in_profit));

        self.unrealized_profit
            .height
            .insert(height, height_state.unrealized_profit as f32);

        self.unrealized_loss
            .height
            .insert(height, height_state.unrealized_loss as f32);

        if is_date_last_block {
            self.supply_in_profit
                .date
                .insert(date, sats_to_btc(date_state.supply_in_profit));

            self.unrealized_profit
                .date
                .insert(date, date_state.unrealized_profit as f32);

            self.unrealized_loss
                .date
                .insert(date, date_state.unrealized_loss as f32);
        }
    }

    #[inline]
    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.supply_in_profit.height,
            &self.unrealized_profit.height,
            &self.unrealized_loss.height,
        ]
    }

    #[inline]
    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.supply_in_profit.date,
            &self.unrealized_profit.date,
            &self.unrealized_loss.date,
        ]
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.supply_in_profit,
            &self.unrealized_profit,
            &self.unrealized_loss,
        ]
    }
}
