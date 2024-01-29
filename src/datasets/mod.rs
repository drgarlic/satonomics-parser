mod date;
mod height;
mod price;

use chrono::NaiveDate;
pub use date::*;
pub use height::*;
pub use price::*;

pub struct AllDatasets {
    pub date: DateDatasets,
    pub height: HeightDatasets,
    pub price: PriceDatasets,
}

impl AllDatasets {
    pub fn import() -> color_eyre::Result<Self> {
        let path = "./datasets";

        Ok(Self {
            date: DateDatasets::import(path)?,
            height: HeightDatasets::import(path)?,
            price: PriceDatasets::import(path)?,
        })
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.date.export()?;
        self.height.export()?;
        self.price.export()?;

        Ok(())
    }

    pub fn export_if_needed(&self, height: usize, date: NaiveDate) -> color_eyre::Result<()> {
        self.date.export_if_needed(date)?;
        self.height.export_if_needed(height)?;
        self.price.export()?;

        Ok(())
    }
}
