use crate::{
    bitcoin::{sats_to_btc, ONE_YEAR_IN_BLOCK_TIME},
    datasets::ProcessedBlockData,
    parse::{AnyBiMap, AnyHeightMap, BiMap},
    utils::ONE_YEAR_IN_DAYS,
};

use super::{AnyDataset, GenericDataset, MinInitialState};

pub struct TransactionDataset {
    min_initial_state: MinInitialState,

    pub count: BiMap<usize>,
    pub volume: BiMap<f32>,

    pub annualized_volume: BiMap<Option<f32>>,
    pub velocity: BiMap<f32>,
}

impl TransactionDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            count: BiMap::new_bin(&f("transaction_count")),
            volume: BiMap::new_bin(&f("transaction_volume")),

            annualized_volume: BiMap::new_bin(&f("annualized_transaction_volume")),
            velocity: BiMap::new_bin(&f("transaction_velocity")),
        };

        s.min_initial_state
            .eat(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }
}

impl GenericDataset for TransactionDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height,
            date,
            sats_sent,
            transaction_count,
            is_date_last_block,
            date_first_height,
            ..
        }: &ProcessedBlockData,
    ) {
        self.count.height.insert(height, transaction_count);
        self.volume.height.insert(height, sats_to_btc(sats_sent));

        self.annualized_volume.height.insert(
            height,
            height
                .checked_sub(ONE_YEAR_IN_BLOCK_TIME)
                .map(|from| self.volume.height.sum(from..=height)),
        );

        // self.velocity.set_height(
        //     self.annualized_volume
        //         .height
        //         .divide(&circulating_supply.height),
        // );
        //     .set_height(self.volume.height.last_x_sum(ONE_YEAR_IN_BLOCK_TIME));

        if is_date_last_block {
            self.count
                .date
                .insert(date, self.count.height.sum(date_first_height..=height));

            self.volume
                .date
                .insert(date, self.volume.height.sum(date_first_height..=height));

            // first_height: &self.date_metadata.first_height,
            //         last_height: &self.date_metadata.last_height,
        }
    }
}

impl AnyDataset for TransactionDataset {
    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count.height, &self.volume.height]
    }

    // fn prepare(
    //     &self,
    //     &ExportData {
    //         convert_last_height_to_date,
    //         convert_sum_heights_to_date,
    //         ..
    //     }: &ExportData,
    // ) {
    // self.count.compute_date(convert_sum_heights_to_date);
    // self.volume.compute_date(convert_sum_heights_to_date);

    // self.annualized_volume
    //     .set_height(self.volume.height.last_x_sum(ONE_YEAR_IN_BLOCK_TIME));
    // self.annualized_volume
    //     .set_date(self.volume.date.last_x_sum(ONE_YEAR_IN_DAYS));
    // }

    // fn compute(
    //     &self,
    //     &ExportData {
    //         circulating_supply, ..
    //     }: &ExportData,
    // ) {
    // self.velocity.set_height(
    //     self.annualized_volume
    //         .height
    //         .divide(&circulating_supply.height),
    // );
    // self.velocity
    //     .set_date(self.annualized_volume.date.divide(&circulating_supply.date));
    // }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.count,
            &self.volume,
            &self.annualized_volume,
            &self.velocity,
        ]
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
