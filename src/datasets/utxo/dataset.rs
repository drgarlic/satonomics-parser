use itertools::Itertools;

use crate::{
    datasets::{
        AnyDataset, AnyDatasetGroup, GenericDataset, MinInitialState, ProcessedBlockData,
        SubDataset,
    },
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
        let name = id.str();

        let folder_path = format!("{parent_path}/{name}");

        let s = Self {
            min_initial_state: MinInitialState::default(),
            id,
            subs: SubDataset::import(&folder_path)?,
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }
}

impl GenericDataset for UTXODataset {
    fn insert_block_data(&self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData {
            block_path_to_received_data,
            block_path_to_spent_data,
            block_price,
            date,
            date_price,
            height,
            is_date_last_block,
            states,
            utxo_cohorts_one_shot_states,
            utxo_cohorts_received_states,
            utxo_cohorts_sent_states,
            ..
        } = processed_block_data;

        let needs_price_paid_data = self.subs.price_paid.should_insert(height, date);
        let needs_unrealized_data = self.subs.unrealized.should_insert(height, date);
        let needs_realized_data = self.subs.realized.should_insert(height, date);
        let needs_input_data = self.subs.input.should_insert(height, date);
        let needs_output_data = self.subs.output.should_insert(height, date);
        let needs_utxo_data = self.subs.utxo.should_insert(height, date);

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

    // fn prepare(&self, export_data: &ExportData) {
    //     self.subs
    //         .to_vec()
    //         .into_iter()
    //         .for_each(|d| d.prepare(export_data))
    // }

    // fn compute(&self, export_data: &ExportData) {
    //     self.subs
    //         .to_vec()
    //         .into_iter()
    //         .for_each(|d| d.compute(export_data))
    // }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_inserted_height_map_vec())
            .collect_vec()
    }

    fn to_any_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_inserted_date_map_vec())
            .collect_vec()
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_bi_map_vec())
            .collect_vec()
    }

    fn to_any_exported_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_date_map_vec())
            .collect_vec()
    }

    fn to_any_exported_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.subs
            .to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_height_map_vec())
            .collect_vec()
    }
}
