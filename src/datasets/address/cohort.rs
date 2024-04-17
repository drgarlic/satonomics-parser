use chrono::NaiveDate;
use itertools::Itertools;

use crate::{
    datasets::{AnyDataset, AnyDatasetGroup, MinInitialState, ProcessedBlockData, SubDataset},
    parse::{AddressSplit, AnyBiMap, AnyDateMap, AnyHeightMap},
    states::AddressCohortDurableStates,
};

use super::cohort_metadata::MetadataDataset;

pub struct CohortDataset {
    min_initial_state: MinInitialState,

    split: AddressSplit,

    metadata: MetadataDataset,

    pub all: SubDataset,
    illiquid: SubDataset,
    liquid: SubDataset,
    highly_liquid: SubDataset,
}

impl CohortDataset {
    pub fn import(
        parent_path: &str,
        name: Option<&str>,
        split: AddressSplit,
    ) -> color_eyre::Result<Self> {
        let folder_path = {
            if let Some(name) = name {
                format!("{parent_path}/{name}")
            } else {
                parent_path.to_owned()
            }
        };

        let f = |s: &str| {
            if let Some(name) = name {
                format!("{parent_path}/{s}/{name}")
            } else {
                format!("{parent_path}/{s}")
            }
        };

        let mut s = Self {
            min_initial_state: MinInitialState::default(),

            split,

            metadata: MetadataDataset::import(&folder_path)?,
            all: SubDataset::import(&folder_path)?,
            illiquid: SubDataset::import(&f("illiquid"))?,
            liquid: SubDataset::import(&f("liquid"))?,
            highly_liquid: SubDataset::import(&f("highly_liquid"))?,
        };

        s.min_initial_state
            .consume(MinInitialState::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn sub_datasets_vec(&self) -> Vec<&SubDataset> {
        vec![&self.all, &self.illiquid, &self.liquid, &self.highly_liquid]
    }

    pub fn needs_metadata(&self, date: NaiveDate, height: usize) -> bool {
        self.metadata.should_insert_date(date) || self.metadata.should_insert_height(height)
    }

    pub fn needs_utxo_data(&self, date: NaiveDate, height: usize) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.utxo.should_insert(height, date))
    }

    pub fn needs_supply_data(&self, date: NaiveDate, height: usize) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.supply.should_insert(height, date))
    }

    pub fn needs_price_paid_data(&self, date: NaiveDate, height: usize) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.price_paid.should_insert(height, date))
    }

    fn needs_realized_data(&self, date: NaiveDate, height: usize) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.realized.should_insert(height, date))
    }

    fn needs_unrealized_data(&self, date: NaiveDate, height: usize) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.unrealized.should_insert(height, date))
    }

    fn needs_input_data(&self, date: NaiveDate, height: usize) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.input.should_insert(height, date))
    }

    fn needs_output_data(&self, date: NaiveDate, height: usize) -> bool {
        self.sub_datasets_vec()
            .iter()
            .any(|sub| sub.output.should_insert(height, date))
    }

    fn insert_realized_data(&mut self, processed_block_data: &ProcessedBlockData) {
        let split_realized_state = processed_block_data
            .address_cohorts_realized_states
            .as_ref()
            .unwrap()
            .get_state(&self.split)
            .unwrap();

        self.all
            .realized
            .insert(processed_block_data, &split_realized_state.all);

        self.illiquid
            .realized
            .insert(processed_block_data, &split_realized_state.illiquid);

        self.liquid
            .realized
            .insert(processed_block_data, &split_realized_state.liquid);

        self.highly_liquid
            .realized
            .insert(processed_block_data, &split_realized_state.highly_liquid);
    }

    fn insert_metadata(&mut self, processed_block_data: &ProcessedBlockData) {
        let address_count = processed_block_data
            .states
            .address_cohorts_durable_states
            .get_state(&self.split)
            .unwrap()
            .address_count;

        self.metadata.insert(processed_block_data, address_count);
    }

    fn insert_supply_data(
        &mut self,
        processed_block_data: &ProcessedBlockData,
        liquidity_split_state: &AddressCohortDurableStates,
    ) {
        self.all.supply.insert(
            processed_block_data,
            &liquidity_split_state.split.all.supply_state,
        );

        self.illiquid.supply.insert(
            processed_block_data,
            &liquidity_split_state.split.illiquid.supply_state,
        );

        self.liquid.supply.insert(
            processed_block_data,
            &liquidity_split_state.split.liquid.supply_state,
        );

        self.highly_liquid.supply.insert(
            processed_block_data,
            &liquidity_split_state.split.highly_liquid.supply_state,
        );
    }

    fn insert_utxo_data(
        &mut self,
        processed_block_data: &ProcessedBlockData,
        liquidity_split_state: &AddressCohortDurableStates,
    ) {
        self.all.utxo.insert(
            processed_block_data,
            &liquidity_split_state.split.all.utxo_state,
        );

        self.illiquid.utxo.insert(
            processed_block_data,
            &liquidity_split_state.split.illiquid.utxo_state,
        );

        self.liquid.utxo.insert(
            processed_block_data,
            &liquidity_split_state.split.liquid.utxo_state,
        );

        self.highly_liquid.utxo.insert(
            processed_block_data,
            &liquidity_split_state.split.highly_liquid.utxo_state,
        );
    }

    fn insert_unrealized_data(&mut self, processed_block_data: &ProcessedBlockData) {
        let states = processed_block_data
            .address_cohorts_one_shot_states
            .as_ref()
            .unwrap()
            .get_state(&self.split)
            .unwrap();

        self.all.unrealized.insert(
            processed_block_data,
            &states.all.unrealized_block_state,
            &states.all.unrealized_date_state,
        );

        self.illiquid.unrealized.insert(
            processed_block_data,
            &states.illiquid.unrealized_block_state,
            &states.illiquid.unrealized_date_state,
        );

        self.liquid.unrealized.insert(
            processed_block_data,
            &states.liquid.unrealized_block_state,
            &states.liquid.unrealized_date_state,
        );

        self.highly_liquid.unrealized.insert(
            processed_block_data,
            &states.highly_liquid.unrealized_block_state,
            &states.highly_liquid.unrealized_date_state,
        );
    }

    fn insert_price_paid_data(&mut self, processed_block_data: &ProcessedBlockData) {
        let states = processed_block_data
            .address_cohorts_one_shot_states
            .as_ref()
            .unwrap()
            .get_state(&self.split)
            .unwrap();

        self.all.price_paid.insert(
            processed_block_data,
            &states.all.price_paid_state,
            self.all
                .supply
                .total
                .height
                .get(&processed_block_data.height)
                .unwrap(),
        );

        self.illiquid.price_paid.insert(
            processed_block_data,
            &states.illiquid.price_paid_state,
            self.illiquid
                .supply
                .total
                .height
                .get(&processed_block_data.height)
                .unwrap(),
        );

        self.liquid.price_paid.insert(
            processed_block_data,
            &states.liquid.price_paid_state,
            self.liquid
                .supply
                .total
                .height
                .get(&processed_block_data.height)
                .unwrap(),
        );

        self.highly_liquid.price_paid.insert(
            processed_block_data,
            &states.highly_liquid.price_paid_state,
            self.highly_liquid
                .supply
                .total
                .height
                .get(&processed_block_data.height)
                .unwrap(),
        );
    }

    fn insert_input_data(&mut self, processed_block_data: &ProcessedBlockData) {
        let state = processed_block_data
            .address_cohorts_input_states
            .as_ref()
            .unwrap()
            .get_state(&self.split)
            .unwrap();

        self.all.input.insert(processed_block_data, &state.all);
        self.illiquid
            .input
            .insert(processed_block_data, &state.illiquid);
        self.liquid
            .input
            .insert(processed_block_data, &state.liquid);
        self.highly_liquid
            .input
            .insert(processed_block_data, &state.highly_liquid);
    }

    fn insert_output_data(&mut self, processed_block_data: &ProcessedBlockData) {
        let state = processed_block_data
            .address_cohorts_output_states
            .as_ref()
            .unwrap()
            .get_state(&self.split)
            .unwrap();

        self.all.output.insert(processed_block_data, &state.all);
        self.illiquid
            .output
            .insert(processed_block_data, &state.illiquid);
        self.liquid
            .output
            .insert(processed_block_data, &state.liquid);
        self.highly_liquid
            .output
            .insert(processed_block_data, &state.highly_liquid);
    }

    fn as_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        vec![
            self.all.as_vec(),
            self.illiquid.as_vec(),
            self.liquid.as_vec(),
            self.highly_liquid.as_vec(),
            vec![&self.metadata],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    fn as_mut_vec(&mut self) -> Vec<&mut dyn AnyDataset> {
        vec![
            self.all.as_mut_vec(),
            self.illiquid.as_mut_vec(),
            self.liquid.as_mut_vec(),
            self.highly_liquid.as_mut_vec(),
            vec![&mut self.metadata],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }

    pub fn insert_data(&mut self, processed_block_data: &ProcessedBlockData) {
        let &ProcessedBlockData { height, date, .. } = processed_block_data;

        let needs_metadata = self.needs_metadata(date, height);
        let needs_unrealized_data = self.needs_unrealized_data(date, height);
        let needs_realized = self.needs_realized_data(date, height);
        let needs_price_paid = self.needs_price_paid_data(date, height);
        let needs_supply = needs_price_paid || self.needs_supply_data(date, height);
        let needs_utxo = self.needs_utxo_data(date, height);
        let needs_input = self.needs_input_data(date, height);
        let needs_output = self.needs_output_data(date, height);

        let liquidity_split_processed_address_state = processed_block_data
            .states
            .address_cohorts_durable_states
            .get_state(&self.split);

        if liquidity_split_processed_address_state.is_none() {
            return; // TODO: Check if should panic instead
        }

        let liquidity_split_processed_address_state =
            liquidity_split_processed_address_state.unwrap();

        if needs_metadata {
            self.insert_metadata(processed_block_data);
        }

        if needs_utxo {
            self.insert_utxo_data(
                processed_block_data,
                liquidity_split_processed_address_state,
            );
        }

        if needs_supply {
            self.insert_supply_data(
                processed_block_data,
                liquidity_split_processed_address_state,
            );
        }

        if needs_realized {
            self.insert_realized_data(processed_block_data);
        }

        if needs_unrealized_data {
            self.insert_unrealized_data(processed_block_data);
        }

        // MUST BE after insert_supply
        if needs_price_paid {
            self.insert_price_paid_data(processed_block_data);
        }

        if needs_input {
            self.insert_input_data(processed_block_data);
        }

        if needs_output {
            self.insert_output_data(processed_block_data);
        }
    }
}

impl AnyDataset for CohortDataset {
    fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.as_vec()
            .into_iter()
            .flat_map(|d| d.to_any_height_map_vec())
            .collect_vec()
    }

    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.as_vec()
            .into_iter()
            .flat_map(|d| d.to_any_date_map_vec())
            .collect_vec()
    }

    fn to_any_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        self.as_vec()
            .into_iter()
            .flat_map(|d| d.to_any_bi_map_vec())
            .collect_vec()
    }

    fn to_any_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        self.as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_any_mut_height_map_vec())
            .collect_vec()
    }

    fn to_any_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        self.as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_any_mut_date_map_vec())
            .collect_vec()
    }

    fn to_any_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        self.as_mut_vec()
            .into_iter()
            .flat_map(|d| d.to_any_mut_bi_map_vec())
            .collect_vec()
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
