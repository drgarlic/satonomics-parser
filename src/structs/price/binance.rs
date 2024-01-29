#![allow(dead_code)]

use std::{collections::BTreeMap, path::Path};

use itertools::Itertools;
use nohash_hasher::IntMap;
use serde_json::Value;

use crate::structs::{Json, IMPORTS_FOLDER_PATH};

pub struct Binance;

impl Binance {
    pub fn read_har_file() -> color_eyre::Result<IntMap<u32, f32>> {
        let path_binance_har = Path::new(IMPORTS_FOLDER_PATH).join("binance.har");

        let json: BTreeMap<String, Value> = Json::import(path_binance_har).unwrap_or_default();

        Ok(json
            .get("log")
            .unwrap()
            .as_object()
            .unwrap()
            .get("entries")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| {
                entry
                    .as_object()
                    .unwrap()
                    .get("request")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("url")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .contains("/uiKlines")
            })
            .flat_map(|entry| {
                let response = entry
                    .as_object()
                    .unwrap()
                    .get("response")
                    .unwrap()
                    .as_object()
                    .unwrap();

                let content = response.get("content").unwrap().as_object().unwrap();

                let text = content.get("text").unwrap().as_str().unwrap();

                let arrays: Value = serde_json::from_str(text).unwrap();

                arrays
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|array| {
                        let array = array.as_array().unwrap();

                        let timestamp = (array.first().unwrap().as_u64().unwrap() / 1000) as u32;

                        let price = array
                            .get(4)
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .parse::<f32>()
                            .unwrap();

                        (timestamp, price)
                    })
                    .collect_vec()
            })
            .collect::<IntMap<_, _>>())
    }

    pub fn fetch_1mn_prices() -> color_eyre::Result<IntMap<u32, f32>> {
        let body: Value = reqwest::blocking::get(
            "https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&interval=1m&limit=1000",
        )?
        .json()?;

        Ok(body
            .as_array()
            .unwrap()
            .iter()
            .map(|value| {
                // [timestamp, open, high, low, close, volume, ...]
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
            .collect::<IntMap<_, _>>())
    }
}
