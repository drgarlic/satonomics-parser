use std::collections::HashMap;

use chrono::NaiveDate;
use color_eyre::eyre::Error;

use crate::{
    datasets::AnyDataset,
    parse::{AnyDateMap, DateMap},
    price::Kraken,
};

pub struct DateDataset {
    name: &'static str,
    min_initial_first_unsafe_date: Option<NaiveDate>,
    min_initial_first_unsafe_height: Option<usize>,
    closes: DateMap<f32>,
    kraken_daily: Option<HashMap<String, f32>>,
}

impl DateDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let name = "close";

        let closes = DateMap::new_in_memory_json(&format!("{parent_path}/{name}"));

        let mut s = Self {
            name,
            min_initial_first_unsafe_date: None,
            min_initial_first_unsafe_height: None,
            closes,
            kraken_daily: None,
        };

        s.min_initial_first_unsafe_date = s.compute_min_initial_first_unsafe_date();
        s.min_initial_first_unsafe_height = s.compute_min_initial_first_unsafe_height();

        Ok(s)
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
            let price = self.get_from_daily_kraken(&date.to_string())?;

            self.closes.insert(date, price);

            Ok(price)
        }
    }

    fn get_from_daily_kraken(&mut self, date: &str) -> color_eyre::Result<f32> {
        if self.kraken_daily.is_none() {
            self.kraken_daily.replace(Kraken::fetch_daily_prices()?);
        }

        self.kraken_daily
            .as_ref()
            .unwrap()
            .get(date)
            .cloned()
            .ok_or(Error::msg("Couldn't find date in daily kraken"))
    }
}

impl AnyDataset for DateDataset {
    fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.closes]
    }

    fn name(&self) -> &str {
        self.name
    }

    fn get_min_initial_first_unsafe_date(&self) -> &Option<NaiveDate> {
        &self.min_initial_first_unsafe_date
    }

    fn get_min_initial_first_unsafe_height(&self) -> &Option<usize> {
        &self.min_initial_first_unsafe_height
    }
}
