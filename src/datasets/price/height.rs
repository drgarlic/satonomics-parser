use chrono::{NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use nohash_hasher::IntMap;

use crate::structs::{AnyHeightMap, Binance, HeightMap, Kraken};

pub struct HeightDatasets {
    closes: HeightMap<f32>,
    kraken_1mn: Option<IntMap<u32, f32>>,
    binance_1mn: Option<IntMap<u32, f32>>,
    binance_har: Option<IntMap<u32, f32>>,
}

impl HeightDatasets {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let closes = HeightMap::new_in_memory_json(&format!("{parent_path}/height_to_close"));

        Ok(Self {
            closes,
            binance_1mn: None,
            binance_har: None,
            kraken_1mn: None,
        })
    }

    pub fn get(&mut self, height: usize, timestamp: u32) -> color_eyre::Result<f32> {
        if self.closes.is_height_safe(height) {
            Ok(self.closes.unsafe_inner().get(height).unwrap().to_owned())
        } else {
            let date_time = Utc.timestamp_opt(i64::from(timestamp), 0).unwrap();
            let timestamp = NaiveDateTime::new(
                date_time.date_naive(),
                NaiveTime::from_hms_opt(date_time.hour(), date_time.minute(), 0).unwrap(),
            )
            .timestamp() as u32;

            let price = self.get_from_1mn_kraken(timestamp).unwrap_or_else(|_| {
                self.get_from_1mn_binance(timestamp)
                    .unwrap_or_else(|_| self.get_from_har_binance(timestamp).unwrap_or_else(|_| {
                        panic!(
                            "Can't find price for {height} - {date_time}, please update binance.har file"
                        )
                    }))
            });

            self.closes.insert(height, price);

            Ok(price)
        }
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.closes.export()
    }

    fn get_from_1mn_kraken(&mut self, timestamp: u32) -> color_eyre::Result<f32> {
        Ok(self
            .kraken_1mn
            .get_or_insert(Kraken::fetch_1mn_prices()?)
            .get(&timestamp)
            .cloned()
            .unwrap())
    }

    fn get_from_1mn_binance(&mut self, timestamp: u32) -> color_eyre::Result<f32> {
        Ok(self
            .binance_1mn
            .get_or_insert(Binance::fetch_1mn_prices()?)
            .get(&timestamp)
            .cloned()
            .unwrap())
    }

    fn get_from_har_binance(&mut self, timestamp: u32) -> color_eyre::Result<f32> {
        Ok(self
            .binance_har
            .get_or_insert(Binance::read_har_file()?)
            .get(&timestamp)
            .cloned()
            .unwrap())
    }
}
