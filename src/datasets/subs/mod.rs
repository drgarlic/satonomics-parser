mod input;
mod output;
mod price_paid;
mod realized;
mod supply;
mod unrealized;
mod utxo;

pub use input::*;
pub use output::*;
pub use price_paid::*;
use rayon::prelude::*;
pub use realized::*;
pub use supply::*;
pub use unrealized::*;
pub use utxo::*;

use itertools::Itertools;

use crate::{
    datasets::AnyDataset,
    parse::{AnyDateMap, AnyExportableMap, AnyHeightMap},
};

use super::{ExportData, MinInitialState};

// Doesn't impl Datasets as insert aren't generic
pub struct SubDataset {
    min_initial_state: MinInitialState,

    pub input: InputSubDataset,
    pub output: OutputSubDataset,
    pub price_paid: PricePaidSubDataset,
    pub realized: RealizedSubDataset,
    pub supply: SupplySubDataset,
    pub unrealized: UnrealizedSubDataset,
    pub utxo: UTXOSubDataset,
}

impl SubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let s = Self {
            min_initial_state: MinInitialState::default(),

            input: InputSubDataset::import(parent_path)?,
            output: OutputSubDataset::import(parent_path)?,
            price_paid: PricePaidSubDataset::import(parent_path)?,
            realized: RealizedSubDataset::import(parent_path)?,
            supply: SupplySubDataset::import(parent_path)?,
            unrealized: UnrealizedSubDataset::import(parent_path)?,
            utxo: UTXOSubDataset::import(parent_path)?,
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }

    fn to_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        vec![
            &self.price_paid,
            &self.realized,
            &self.supply,
            &self.unrealized,
            &self.utxo,
            &self.input,
            &self.output,
        ]
    }
}

impl AnyDataset for SubDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn prepare(&self, export_data: &ExportData) {
        self.to_vec()
            .into_par_iter()
            .for_each(|d| d.prepare(export_data))
    }

    fn compute(&self, export_data: &ExportData) {
        self.to_vec()
            .into_par_iter()
            .for_each(|d| d.compute(export_data))
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

    fn to_any_exported_bi_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        self.to_vec()
            .into_iter()
            .flat_map(|d| d.to_any_exported_bi_map_vec())
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
}
