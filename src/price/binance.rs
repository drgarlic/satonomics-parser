#![allow(dead_code)]

use std::{collections::BTreeMap, path::Path};

use color_eyre::eyre::ContextCompat;
use itertools::Itertools;
use serde_json::Value;

use crate::io::{Json, IMPORTS_FOLDER_PATH};

pub struct Binance;

impl Binance {
    pub fn read_har_file() -> color_eyre::Result<BTreeMap<u32, f32>> {
        println!("binance: read har file");

        let path_binance_har = Path::new(IMPORTS_FOLDER_PATH).join("binance.har");

        let json: BTreeMap<String, Value> =
            Json::import(path_binance_har.to_str().unwrap()).unwrap_or_default();

        Ok(json
            .get("log")
            .context("Expect object to have log attribute")?
            .as_object()
            .context("Expect to be an object")?
            .get("entries")
            .context("Expect object to have entries")?
            .as_array()
            .context("Expect to be an array")?
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

                let text = content.get("text");

                if text.is_none() {
                    return vec![];
                }

                let text = text.unwrap().as_str().unwrap();

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
            .collect::<BTreeMap<_, _>>())
    }

    pub fn fetch_1mn_prices() -> color_eyre::Result<BTreeMap<u32, f32>> {
        println!("binance: fetch 1mn");

        let body: Value = reqwest::blocking::get(
            "https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&interval=1m&limit=1000",
        )?
        .json()?;

        Ok(body
            .as_array()
            .context("Expect to be an array")?
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
            .collect::<BTreeMap<_, _>>())
    }
}
