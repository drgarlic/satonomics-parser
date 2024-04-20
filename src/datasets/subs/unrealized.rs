use crate::{
    bitcoin::sats_to_btc,
    datasets::{AnyDataset, MinInitialState, ProcessedBlockData},
    parse::{AnyBiMap, BiMap},
    states::UnrealizedState,
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

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            supply_in_profit: BiMap::new_bin(1, &f("supply_in_profit")),
            unrealized_profit: BiMap::new_bin(1, &f("unrealized_profit")),
            unrealized_loss: BiMap::new_bin(1, &f("unrealized_loss")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &ProcessedBlockData {
            height,
            date,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
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

        if is_date_last_block {
            let date_state = date_state.as_ref().unwrap();

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

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.supply_in_profit,
            &self.unrealized_profit,
            &self.unrealized_loss,
        ]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.supply_in_profit,
            &mut self.unrealized_profit,
            &mut self.unrealized_loss,
        ]
    }
}
