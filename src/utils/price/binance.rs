#![allow(dead_code)]

use std::{collections::HashMap, path::Path};

use itertools::Itertools;
use serde_json::Value;

use crate::utils::{import_json_map, INPUTS_FOLDER_RAW_PATH};

pub fn read_binance_har_file() -> color_eyre::Result<HashMap<u32, f32>> {
    let path_binance_har = Path::new(INPUTS_FOLDER_RAW_PATH).join("binance.har");

    let json = import_json_map::<Value>(path_binance_har.as_path(), true)?;

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

                    let timestamp = (array.get(0).unwrap().as_u64().unwrap() / 1000) as u32;

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
        .collect::<HashMap<_, _>>())
}
