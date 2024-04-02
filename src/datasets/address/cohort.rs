use chrono::NaiveDate;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    datasets::{
        AnyDataset, AnyDatasetGroup, ExportData, GenericDataset, MinInitialState,
        ProcessedBlockData, SubDataset,
    },
    parse::{AnyDateMap, AnyExportableMap, AnyHeightMap, RawAddressSplit},
    states::LiquiditySplitProcessedAddressState,
};

use super::cohort_metadata::MetadataDataset;

pub struct CohortDataset {
    min_initial_state: MinInitialState,

    split: RawAddressSplit,

    metadata: MetadataDataset,

    all: SubDataset,
    illiquid: SubDataset,
    liquid: SubDataset,
    highly_liquid: SubDataset,
}

impl CohortDataset {
    pub fn import(
        parent_path: &str,
        name: &str,
        split: RawAddressSplit,
    ) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/{name}");

        let f = |s: &str| format!("{parent_path}/{s}/{name}");

        let s = Self {
            min_initial_state: MinInitialState::default(),

            split,

            metadata: MetadataDataset::import(&folder_path)?,
            all: SubDataset::import(&folder_path)?,
            illiquid: SubDataset::import(&f("illiquid"))?,
            liquid: SubDataset::import(&f("liquid"))?,
            highly_liquid: SubDataset::import(&f("highly_liquid"))?,
        };

        s.min_initial_state.compute_from_dataset(&s);

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

    fn insert_realized_data(&self, processed_block_data: &ProcessedBlockData) {
        let split_realized_state = processed_block_data
            .split_realized_states
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

    fn insert_metadata(
        &self,
        &ProcessedBlockData {
            height,
            date,
            is_date_last_block,
            states,
            ..
        }: &ProcessedBlockData,
    ) {
        let address_count = states
            .split_address
            .get_state(&self.split)
            .unwrap()
            .address_count;

        self.metadata
            .address_count
            .height
            .insert(height, address_count);

        if is_date_last_block {
            self.metadata.address_count.date.insert(date, address_count);
        }
    }

    fn insert_supply_data(
        &self,
        processed_block_data: &ProcessedBlockData,
        liquidity_split_state: &LiquiditySplitProcessedAddressState,
    ) {
        self.all.supply.insert(
            processed_block_data,
            &liquidity_split_state.split.all.supply,
        );

        self.illiquid.supply.insert(
            processed_block_data,
            &liquidity_split_state.split.illiquid.supply,
        );

        self.liquid.supply.insert(
            processed_block_data,
            &liquidity_split_state.split.liquid.supply,
        );

        self.highly_liquid.supply.insert(
            processed_block_data,
            &liquidity_split_state.split.highly_liquid.supply,
        );
    }

    fn insert_utxo_data(
        &self,
        processed_block_data: &ProcessedBlockData,
        liquidity_split_state: &LiquiditySplitProcessedAddressState,
    ) {
        self.all.utxo.insert(
            processed_block_data,
            &liquidity_split_state.split.all.utxos_metadata,
        );

        self.illiquid.utxo.insert(
            processed_block_data,
            &liquidity_split_state.split.illiquid.utxos_metadata,
        );

        self.liquid.utxo.insert(
            processed_block_data,
            &liquidity_split_state.split.liquid.utxos_metadata,
        );

        self.highly_liquid.utxo.insert(
            processed_block_data,
            &liquidity_split_state.split.highly_liquid.utxos_metadata,
        );
    }

    fn insert_unrealized_data(&self, processed_block_data: &ProcessedBlockData) {
        let height_state = processed_block_data
            .split_unrealized_states_height
            .as_ref()
            .unwrap()
            .get_state(&self.split)
            .unwrap();

        let date_state = processed_block_data
            .split_unrealized_states_height
            .as_ref()
            .unwrap()
            .get_state(&self.split)
            .unwrap();

        self.all
            .unrealized
            .insert(processed_block_data, &height_state.all, &date_state.all);

        self.illiquid.unrealized.insert(
            processed_block_data,
            &height_state.illiquid,
            &date_state.illiquid,
        );

        self.liquid.unrealized.insert(
            processed_block_data,
            &height_state.liquid,
            &date_state.liquid,
        );

        self.highly_liquid.unrealized.insert(
            processed_block_data,
            &height_state.highly_liquid,
            &date_state.highly_liquid,
        );
    }

    fn insert_price_paid_data(&self, processed_block_data: &ProcessedBlockData) {
        let state = processed_block_data
            .split_price_paid_states
            .as_ref()
            .unwrap()
            .get_state(&self.split)
            .unwrap();

        self.all
            .price_paid
            .insert(processed_block_data, &state.all, &self.all.supply.total);
        self.illiquid.price_paid.insert(
            processed_block_data,
            &state.illiquid,
            &self.illiquid.supply.total,
        );
        self.liquid.price_paid.insert(
            processed_block_data,
            &state.liquid,
            &self.liquid.supply.total,
        );
        self.highly_liquid.price_paid.insert(
            processed_block_data,
            &state.highly_liquid,
            &self.highly_liquid.supply.total,
        );
    }

    fn insert_input_data(&self, processed_block_data: &ProcessedBlockData) {
        let state = processed_block_data
            .split_input_states
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

    fn insert_output_data(&self, processed_block_data: &ProcessedBlockData) {
        let state = processed_block_data
            .split_output_states
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

    fn to_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        vec![
            self.all.to_vec(),
            self.illiquid.to_vec(),
            self.liquid.to_vec(),
            self.highly_liquid.to_vec(),
            vec![&self.metadata],
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}

impl GenericDataset for CohortDataset {
    fn insert_block_data(&self, processed_block_data: &ProcessedBlockData) {
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
            .split_address
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
    fn prepare(&self, export_data: &ExportData) {
        self.to_vec()
            .into_par_iter()
            .for_each(|d| d.prepare(export_data));
    }

    fn compute(&self, export_data: &ExportData) {
        self.to_vec()
            .into_par_iter()
            .for_each(|d| d.compute(export_data));
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        self.to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_inserted_height_map_vec())
            .collect_vec()
    }

    fn to_any_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        self.to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_inserted_date_map_vec())
            .collect_vec()
    }

    fn to_any_exported_date_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        self.to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_date_map_vec())
            .collect_vec()
    }

    fn to_any_exported_height_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        self.to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_height_map_vec())
            .collect_vec()
    }

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        self.to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_bi_map_vec())
            .collect_vec()
    }

    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }
}
