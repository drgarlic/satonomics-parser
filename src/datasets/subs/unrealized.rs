use std::ops::Add;

use crate::{
    bitcoin::sats_to_btc,
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap},
};

pub struct UnrealizedSubDataset {
    min_initial_state: MinInitialState,

    supply_in_profit: BiMap<f32>,
    unrealized_profit: BiMap<f32>,
    unrealized_loss: BiMap<f32>,
}

impl UnrealizedSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            supply_in_profit: BiMap::new_on_disk_bin(&f("supply_in_profit")),
            unrealized_profit: BiMap::new_on_disk_bin(&f("unrealized_profit")),
            unrealized_loss: BiMap::new_on_disk_bin(&f("unrealized_loss")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }

    pub fn insert(
        &self,
        &ProcessedBlockData { height, date, .. }: &ProcessedBlockData,
        block_state: &UnrealizedState,
        date_state: &Option<UnrealizedState>,
    ) {
        self.supply_in_profit
            .height
            .insert(height, sats_to_btc(block_state.supply_in_profit));

        self.unrealized_profit
            .height
            .insert(height, block_state.unrealized_profit);

        self.unrealized_loss
            .height
            .insert(height, block_state.unrealized_loss);

        if let Some(date_state) = date_state {
            self.supply_in_profit
                .date
                .insert(date, sats_to_btc(date_state.supply_in_profit));

            self.unrealized_profit
                .date
                .insert(date, date_state.unrealized_profit);

            self.unrealized_loss
                .date
                .insert(date, date_state.unrealized_loss);
        }
    }
}

impl AnyDataset for UnrealizedSubDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.supply_in_profit.date,
            &self.unrealized_profit.date,
            &self.unrealized_loss.date,
        ]
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.supply_in_profit.height,
            &self.unrealized_profit.height,
            &self.unrealized_loss.height,
        ]
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.supply_in_profit,
            &self.unrealized_profit,
            &self.unrealized_loss,
        ]
    }
}

// ---
// STATE
// ---

#[derive(Debug, Default)]
pub struct UnrealizedState {
    supply_in_profit: u64,
    unrealized_profit: f32,
    unrealized_loss: f32,
}

impl UnrealizedState {
    #[inline]
    pub fn iterate(&mut self, price_then: f32, price_now: f32, sat_amount: u64, btc_amount: f32) {
        if price_then < price_now {
            self.unrealized_profit += btc_amount * (price_now - price_then);
            self.supply_in_profit += sat_amount;
        } else if price_then > price_now {
            self.unrealized_loss += btc_amount * (price_then - price_now);
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
