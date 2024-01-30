mod date;
mod height;

use chrono::NaiveDate;
use date::*;
use height::*;

use super::AnyDatasets;

pub struct PriceDatasets {
    date: DateDataset,
    height: HeightDataset,
}

impl PriceDatasets {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let dir = format!("{parent_path}/price");

        Ok(Self {
            date: DateDataset::import(&dir)?,
            height: HeightDataset::import(&dir)?,
        })
    }

    pub fn date_to_close(&mut self, date: NaiveDate) -> color_eyre::Result<f32> {
        self.date.get(date)
    }

    pub fn height_to_close(&mut self, height: usize, timestamp: u32) -> color_eyre::Result<f32> {
        self.height.get(height, timestamp)
    }
}

impl AnyDatasets for PriceDatasets {
    fn to_vec(&self) -> Vec<&(dyn super::AnyDataset + Send + Sync)> {
        vec![&self.date, &self.height]
    }
}
