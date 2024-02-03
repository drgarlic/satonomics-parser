#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};

use color_eyre::eyre::ContextCompat;
use serde_json::Value;

use crate::utils::timestamp_to_naive_date;

pub struct Kraken;

impl Kraken {
    pub fn fetch_1mn_prices() -> color_eyre::Result<BTreeMap<u32, f32>> {
        println!("kraken: fetch 1mn");

        let body: Value =
            reqwest::blocking::get("https://api.kraken.com/0/public/OHLC?pair=XBTUSD&interval=1")?
                .json()?;

        Ok(body
            .as_object()
            .context("Expect to be an object")?
            .get("result")
            .context("Expect object to have result")?
            .as_object()
            .context("Expect to be an object")?
            .get("XXBTZUSD")
            .context("Expect to have XXBTZUSD")?
            .as_array()
            .context("Expect to be an array")?
            .iter()
            .map(|value| {
                let array = value.as_array().unwrap();

                let timestamp = array.first().unwrap().as_u64().unwrap() as u32;

                let price = array
                    .get(4)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();

                (timestamp, price)
            })
            .collect::<BTreeMap<_, _>>())
    }

    pub fn fetch_daily_prices() -> color_eyre::Result<HashMap<String, f32>> {
        println!("fetch kraken daily");

        let body: Value = reqwest::blocking::get(
            "https://api.kraken.com/0/public/OHLC?pair=XBTUSD&interval=1440",
        )?
        .json()?;

        Ok(body
            .as_object()
            .context("Expect to be an object")?
            .get("result")
            .context("Expect object to have result")?
            .as_object()
            .context("Expect to be an object")?
            .get("XXBTZUSD")
            .context("Expect to have XXBTZUSD")?
            .as_array()
            .context("Expect to be an array")?
            .iter()
            .map(|value| {
                let array = value.as_array().unwrap();

                let date = timestamp_to_naive_date(array.first().unwrap().as_u64().unwrap() as u32)
                    .to_string();

                let price = array
                    .get(4)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();

                (date, price)
            })
            .collect::<HashMap<_, _>>())
    }
}
