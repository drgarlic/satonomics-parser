use std::collections::BTreeMap;

use chrono::{NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use color_eyre::eyre::Error;

use crate::{
    datasets::{AnyDataset, GenericDataset, MinInitialState},
    parse::{AnyExportableMap, AnyHeightMap, HeightMap},
    price::{Binance, Kraken},
};

pub struct HeightDataset {
    min_initial_state: MinInitialState,

    kraken_1mn: Option<BTreeMap<u32, f32>>,
    binance_1mn: Option<BTreeMap<u32, f32>>,
    binance_har: Option<BTreeMap<u32, f32>>,

    closes: HeightMap<f32>,
}

impl HeightDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let name = "close";

        let closes = HeightMap::new_in_memory_json(&format!("{parent_path}/{name}"));

        let s = Self {
            min_initial_state: MinInitialState::default(),

            binance_1mn: None,
            binance_har: None,
            kraken_1mn: None,

            closes,
        };

        s.min_initial_state.compute_from_dataset(&s);

        Ok(s)
    }

    pub fn get(&mut self, height: usize, timestamp: u32) -> color_eyre::Result<f32> {
        {
            let inner = self.closes.inner.lock();

            let closes = inner.as_ref().unwrap();

            if height < closes.len() - 1 {
                return Ok(closes.get(height).unwrap().to_owned());
            }
        };

        let date_time = Utc.timestamp_opt(i64::from(timestamp), 0).unwrap();
        let timestamp = NaiveDateTime::new(
            date_time.date_naive(),
            NaiveTime::from_hms_opt(date_time.hour(), date_time.minute(), 0).unwrap(),
        )
        .and_utc()
        .timestamp() as u32;

        let price = self.get_from_1mn_kraken(timestamp).unwrap_or_else(|_| {
                self.get_from_1mn_binance(timestamp)
                    .unwrap_or_else(|_| self.get_from_har_binance(timestamp).unwrap_or_else(|_| {
                        panic!(
                            "Can't find price for {height} - {date_time} - {timestamp}, please update binance.har file"
                        )
                    }))
            });

        self.closes.insert(height, price);

        Ok(price)
    }

    fn get_from_1mn_kraken(&mut self, timestamp: u32) -> color_eyre::Result<f32> {
        if self.kraken_1mn.is_none() {
            self.kraken_1mn.replace(Kraken::fetch_1mn_prices()?);
        }

        self.kraken_1mn
            .as_ref()
            .unwrap()
            .get(&timestamp)
            .cloned()
            .ok_or(Error::msg("Couldn't find timestamp in 1mn kraken"))
    }

    fn get_from_1mn_binance(&mut self, timestamp: u32) -> color_eyre::Result<f32> {
        if self.binance_1mn.is_none() {
            self.binance_1mn.replace(Binance::fetch_1mn_prices()?);
        }

        self.binance_1mn
            .as_ref()
            .unwrap()
            .get(&timestamp)
            .cloned()
            .ok_or(Error::msg("Couldn't find timestamp in 1mn binance"))
    }

    fn get_from_har_binance(&mut self, timestamp: u32) -> color_eyre::Result<f32> {
        if self.binance_har.is_none() {
            self.binance_har.replace(Binance::read_har_file()?);
        }

        self.binance_har
            .as_ref()
            .unwrap()
            .get(&timestamp)
            .cloned()
            .ok_or(Error::msg("Couldn't find timestamp in har binance"))
    }
}

impl GenericDataset for HeightDataset {}

impl AnyDataset for HeightDataset {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.closes]
    }

    fn to_any_exported_height_map_vec(&self) -> Vec<&(dyn AnyExportableMap + Send + Sync)> {
        vec![&self.closes]
    }
}
