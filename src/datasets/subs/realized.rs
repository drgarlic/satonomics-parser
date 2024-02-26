use chrono::NaiveDate;
use itertools::Itertools;

use crate::{
    datasets::ProcessedBlockData,
    structs::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap},
};

/// NOTE: Fees not taken into account
pub struct RealizedSubDataset {
    profit: BiMap<f32>,
    loss: BiMap<f32>,
}

#[derive(Debug, Default)]
pub struct RealizedState {
    pub realized_profit: f32,
    pub realized_loss: f32,
}

impl RealizedState {
    pub fn iterate(&mut self, realized_profit: f32, realized_loss: f32) {
        self.realized_profit += realized_profit;
        self.realized_loss += realized_loss;
    }
}

impl RealizedSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/realized");
        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            profit: BiMap::new_on_disk_bin(&f("profit")),
            loss: BiMap::new_on_disk_bin(&f("loss")),
        })
    }

    pub fn insert(
        &self,
        &ProcessedBlockData {
            date,
            height,
            is_date_last_block,
            first_date_height,
            ..
        }: &ProcessedBlockData,
        height_state: &RealizedState,
    ) {
        self.profit
            .height
            .insert(height, height_state.realized_profit);
        self.loss.height.insert(height, height_state.realized_loss);

        if is_date_last_block {
            let realized_profit = self.profit.height.sum_last_day_values(first_date_height);

            self.profit.date.insert(date, realized_profit);

            let realized_loss = self.loss.height.sum_last_day_values(first_date_height);

            self.loss.date.insert(date, realized_loss);
        }
    }

    pub fn are_date_and_height_safe(&self, date: NaiveDate, height: usize) -> bool {
        self.to_vec()
            .iter()
            .any(|bi| bi.are_date_and_height_safe(date, height))
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.to_vec()
            .into_iter()
            .map(|bi| &bi.height as &(dyn AnyHeightMap + Send + Sync))
            .collect_vec()
    }

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.to_vec()
            .into_iter()
            .map(|bi| &bi.date as &(dyn AnyDateMap + Send + Sync))
            .collect_vec()
    }

    pub fn to_vec(&self) -> Vec<&BiMap<f32>> {
        vec![&self.profit, &self.loss]
    }
}
