use std::collections::HashMap;

use chrono::NaiveDate;

use crate::structs::{AnyDateMap, DateMap, Kraken};

pub struct DateDatasets {
    closes: DateMap<f32>,
    kraken_daily: Option<HashMap<String, f32>>,
}

impl DateDatasets {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let closes = DateMap::new_in_memory_json(&format!("{parent_path}/date_to_close"));

        Ok(Self {
            closes,
            kraken_daily: None,
        })
    }

    pub fn get(&mut self, date: NaiveDate) -> color_eyre::Result<f32> {
        if self.closes.is_date_safe(date) {
            Ok(self
                .closes
                .unsafe_inner()
                .get(&date.to_string())
                .unwrap()
                .to_owned())
        } else {
            let price = self
                .get_from_daily_kraken(&date.to_string())
                .unwrap_or_else(|_| panic!("Can't find price for {date}"));

            self.closes.insert(date, price);

            Ok(price)
        }
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.closes.export()
    }

    fn get_from_daily_kraken(&mut self, date: &str) -> color_eyre::Result<f32> {
        Ok(self
            .kraken_daily
            .get_or_insert(Kraken::fetch_daily_prices()?)
            .get(date)
            .cloned()
            .unwrap())
    }
}
