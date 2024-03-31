mod date;
mod height;

use chrono::NaiveDate;
use date::*;
use height::*;

use super::{AnyDatasets, GenericDataset, MinInitialState};

pub struct PriceDatasets {
    min_initial_state: MinInitialState,

    date: DateDataset,
    height: HeightDataset,
}

impl PriceDatasets {
    pub fn import() -> color_eyre::Result<Self> {
        let path = "./price";

        let s = Self {
            min_initial_state: MinInitialState::default(),

            date: DateDataset::import(path)?,
            height: HeightDataset::import(path)?,
        };

        s.min_initial_state.compute_from_datasets(&s);

        Ok(s)
    }

    pub fn date_to_close(&mut self, date: NaiveDate) -> color_eyre::Result<f32> {
        self.date.get(date)
    }

    pub fn height_to_close(&mut self, height: usize, timestamp: u32) -> color_eyre::Result<f32> {
        self.height.get(height, timestamp)
    }
}

impl AnyDatasets for PriceDatasets {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_generic_dataset_vec(&self) -> Vec<&(dyn GenericDataset + Send + Sync)> {
        vec![&self.date, &self.height]
    }
}
