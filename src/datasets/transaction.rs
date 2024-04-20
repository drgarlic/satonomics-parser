use crate::{
    bitcoin::{sats_to_btc, ONE_YEAR_IN_BLOCK_TIME},
    datasets::ProcessedBlockData,
    parse::{AnyBiMap, BiMap},
    utils::ONE_YEAR_IN_DAYS,
};

use super::{AddressDatasets, AnyDataset, MinInitialState};

pub struct TransactionDataset {
    min_initial_state: MinInitialState,

    pub count: BiMap<usize>,
    pub volume: BiMap<f32>,

    pub annualized_volume: BiMap<f32>,
    pub velocity: BiMap<f32>,
}

impl TransactionDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            count: BiMap::new_bin(1, &f("transaction_count")),
            volume: BiMap::_new_bin(1, &f("transaction_volume"), 5),

            annualized_volume: BiMap::new_bin(1, &f("annualized_transaction_volume")),
            velocity: BiMap::new_bin(1, &f("transaction_velocity")),
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert_data(
        &mut self,
        &ProcessedBlockData {
            height,
            date,
            sats_sent,
            transaction_count,
            is_date_last_block,
            date_blocks_range,
            ..
        }: &ProcessedBlockData,
        address_datasets: &AddressDatasets,
    ) {
        let circulating_supply_map = &address_datasets.all.all.supply.total;
        let circulating_supply = circulating_supply_map.height.get(&height).unwrap();

        self.count.height.insert(height, transaction_count);

        self.volume.height.insert(height, sats_to_btc(sats_sent));

        let annualized_volume = self.annualized_volume.height.insert_last_x_sum(
            height,
            &self.volume.height,
            ONE_YEAR_IN_BLOCK_TIME,
        );

        self.velocity
            .height
            .insert(height, annualized_volume / circulating_supply);

        if is_date_last_block {
            self.count.date_insert_sum_range(date, date_blocks_range);

            self.volume.date_insert_sum_range(date, date_blocks_range);

            let annualized_volume = self.annualized_volume.date.insert_last_x_sum(
                date,
                &self.volume.date,
                ONE_YEAR_IN_DAYS,
            );

            self.velocity
                .date
                .insert(date, annualized_volume / circulating_supply);
        }
    }
}

impl AnyDataset for TransactionDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.count,
            &self.volume,
            &self.annualized_volume,
            &self.velocity,
        ]
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.count,
            &mut self.volume,
            &mut self.annualized_volume,
            &mut self.velocity,
        ]
    }
}
