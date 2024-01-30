mod date;
mod height;

use std::fs;

use chrono::NaiveDate;
use date::*;
use height::*;

pub struct PriceDatasets {
    date: DateDatasets,
    height: HeightDatasets,
}

impl PriceDatasets {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let dir = format!("{parent_path}/price");

        fs::create_dir_all(&dir)?;

        Ok(Self {
            date: DateDatasets::import(&dir)?,
            height: HeightDatasets::import(&dir)?,
        })
    }

    pub fn date_to_close(&mut self, date: NaiveDate) -> color_eyre::Result<f32> {
        self.date.get(date)
    }

    pub fn height_to_close(&mut self, height: usize, timestamp: u32) -> color_eyre::Result<f32> {
        self.height.get(height, timestamp)
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.date.export()?;
        self.height.export()?;

        Ok(())
    }
}
