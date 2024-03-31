use crate::{
    bitcoin::{sats_to_btc, ONE_YEAR_IN_BLOCK_TIME},
    datasets::{ExportData, ProcessedBlockData},
    parse::{AnyExportableMap, AnyHeightMap, BiMap},
    utils::ONE_YEAR_IN_DAYS,
};

use super::{AnyDataset, GenericDataset, MinInitialState};

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

        let s = Self {
            min_initial_state: MinInitialState::default(),

            count: BiMap::new_on_disk_bin(&f("transaction_count")),
            volume: BiMap::new_on_disk_bin(&f("transaction_volume")),

            annualized_volume: BiMap::new_on_disk_bin(&f("annualized_transaction_volume")),
            velocity: BiMap::new_on_disk_bin(&f("transaction_velocity")),
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl GenericDataset for TransactionDataset {
    fn insert_block_data(
        &self,
        &ProcessedBlockData {
            height,
            sats_sent,
            transaction_count,
            ..
        }: &ProcessedBlockData,
    ) {
        self.count.insert(height, transaction_count);
        self.volume.insert(height, sats_to_btc(sats_sent));
    }
}

impl AnyDataset for TransactionDataset {
    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count.height, &self.volume.height]
    }

    fn compute(
        &mut self,
        &ExportData {
            circulating_supply,
            last_height_to_date,
            sum_heights_to_date,
            ..
        }: &ExportData,
    ) {
        self.count.compute_date(last_height_to_date);
        self.volume.compute_date(sum_heights_to_date);

        self.annualized_volume
            .set_height(self.volume.height.last_x_sum(ONE_YEAR_IN_BLOCK_TIME));
        self.annualized_volume
            .set_date(self.volume.date.last_x_sum(ONE_YEAR_IN_DAYS));

        self.velocity.set_height(
            self.annualized_volume
                .height
                .divide(&circulating_supply.height),
        );
        self.velocity
            .set_date(self.annualized_volume.date.divide(&circulating_supply.date));
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
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
