mod date;
mod height;
mod price;

use chrono::NaiveDate;
pub use date::*;
pub use height::*;
pub use price::*;

const DATASETS_PATH: &str = "./datasets";

pub struct AllDatasets {
    pub date: DateDatasets,
    pub height: HeightDatasets,
}

impl AllDatasets {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self {
            date: DateDatasets::import()?,
            height: HeightDatasets::import()?,
        })
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.date.export()?;
        self.height.export()?;
        Ok(())
    }

    pub fn export_if_needed(&self, height: usize, date: NaiveDate) -> color_eyre::Result<()> {
        self.date.export_if_needed(date)?;
        self.height.export_if_needed(height)?;
        Ok(())
    }
}
