use itertools::Itertools;

use crate::{
    datasets::{AnyDataset, AnyDatasetGroup, MinInitialState, ProcessedBlockData, SubDataset},
    parse::{AnyBiMap, AnyDateMap, AnyHeightMap},
    states::UTXOCohortId,
};

pub struct UTXODataset {
    min_initial_state: MinInitialState,
    id: UTXOCohortId,

    pub subs: SubDataset,
}

impl UTXODataset {
    pub fn import(parent_path: &str, id: UTXOCohortId) -> color_eyre::Result<Self> {
        let name = id.name();

        let folder_path = format!("{parent_path}/{name}");

        let mut s = Self {
            min_initial_state: MinInitialState::default(),
            id,
            subs: SubDataset::import(&folder_path)?,
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert_data(&mut self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            date,
            height,
            states,
            utxo_cohorts_one_shot_states,
            utxo_cohorts_received_states,
            utxo_cohorts_sent_states,
            ..
        } = processed_block_data;

        if self.subs.supply.should_insert(height, date) {
            self.subs.supply.insert(
                processed_block_data,
                &states
                    .utxo_cohorts_durable_states
                    .get(&self.id)
                    .supply_state,
            );
        }

        if self.subs.utxo.should_insert(height, date) {
            self.subs.utxo.insert(
                processed_block_data,
                &states.utxo_cohorts_durable_states.get(&self.id).utxo_state,
            );
        }

        if self.subs.unrealized.should_insert(height, date) {
            self.subs.unrealized.insert(
                processed_block_data,
                &utxo_cohorts_one_shot_states
                    .get(&self.id)
                    .unrealized_block_state,
                &utxo_cohorts_one_shot_states
                    .get(&self.id)
                    .unrealized_date_state,
            );
        }

        if self.subs.price_paid.should_insert(height, date) {
            self.subs.price_paid.insert(
                processed_block_data,
                &utxo_cohorts_one_shot_states.get(&self.id).price_paid_state,
                self.subs.supply.total.height.get(&height).unwrap(),
            );
        }

        if self.subs.realized.should_insert(height, date) {
            self.subs.realized.insert(
                processed_block_data,
                &utxo_cohorts_sent_states.get(&self.id).realized,
            );
        }

        if self.subs.input.should_insert(height, date) {
            self.subs.input.insert(
                processed_block_data,
                &utxo_cohorts_sent_states.get(&self.id).input,
            );
        }

        if self.subs.output.should_insert(height, date) {
            self.subs.output.insert(
                processed_block_data,
                utxo_cohorts_received_states.get(&self.id),
            );
        }
    }
}

impl AnyDataset for UTXODataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_any_height_map_vec())
            .collect_vec()
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_any_date_map_vec())
            .collect_vec()
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.subs
            .as_vec()
            .into_iter()
            .flat_map(|d| d.to_any_bi_map_vec())
            .collect_vec()
    }

    fn to_any_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_any_mut_height_map_vec())
            .collect_vec()
    }

    fn to_any_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_any_mut_date_map_vec())
            .collect_vec()
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        self.subs
            .as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_any_mut_bi_map_vec())
            .collect_vec()
    }
}
