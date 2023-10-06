use bitcoin_explorer::{BitcoinDB, FBlock, Txid};
use chrono::{offset::Local, Days, NaiveDate};
use itertools::Itertools;
use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    path::Path,
    rc::Rc,
};

use crate::utils::{
    create_group_blocks_by_day_closure, import_json_map, insert_to_saved_map,
    timestamp_to_naive_date,
};

const STH_MAX_AGE: usize = 155;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProcessingBlockMetaData {
    height: usize,
    outputs_by_txids: RefCell<FxHashMap<Txid, RefCell<IntMap<u32, f64>>>>,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct SavingBlockMetaData {
    total: f64,
    in_profit: f64,
}

pub fn process(
    db: &BitcoinDB,
    block_count: usize,
    outputs_folder_rawpath: &str,
) -> color_eyre::Result<()> {
    let path_sth = Path::new(outputs_folder_rawpath).join("sth.json");
    let path_first_block = Path::new(outputs_folder_rawpath).join("first_block.json");
    let path_daily_closes = Path::new("./src/assets/daily.json");

    let closes: HashMap<String, f64> = import_json_map(path_daily_closes, false)?;

    let mut block_metadatas_per_day: VecDeque<Vec<Rc<ProcessingBlockMetaData>>> = VecDeque::new();

    let mut txid_to_block_metadata: FxHashMap<Txid, Rc<ProcessingBlockMetaData>> =
        FxHashMap::default();

    let saved_sth: HashMap<String, SavingBlockMetaData> = import_json_map(&path_sth, true)?;

    let latest_date = saved_sth
        .keys()
        .map(|key| NaiveDate::parse_from_str(key, "%Y-%m-%d").unwrap())
        .max();
    println!("Latest date: {latest_date:?}");

    let starting_date = latest_date.map(|date| {
        date.to_owned()
            .checked_sub_days(Days::new(STH_MAX_AGE as u64 + 1))
            .unwrap()
    });
    println!("Starting date: {starting_date:?}");

    let first_block_indexes: HashMap<String, usize> = import_json_map(&path_first_block, true)?;
    let mut first_block_index: usize = {
        if let Some(starting_date) = starting_date {
            first_block_indexes
                .get(&starting_date.to_string())
                .unwrap_or(&0)
                .to_owned()
        } else {
            0
        }
    };

    println!("First block index: {first_block_index}");

    let mut previous_daily_close: f64 = 0.0;

    db.iter_block::<FBlock>(first_block_index.to_owned(), block_count)
        .batching(create_group_blocks_by_day_closure())
        .try_for_each(|blocks| -> color_eyre::Result<()> {
            let date = timestamp_to_naive_date(blocks.first().unwrap().header.time);

            let date_str = date.to_string();

            insert_to_saved_map(
                &path_first_block,
                date_str.to_owned(),
                first_block_index
            )?;

            first_block_index += blocks.len();

            if starting_date.map(|starting_date| date < starting_date).unwrap_or(false) {
                println!("{:?} - Skipping {date} ({} blocks)...", Local::now(), blocks.len());

                return Ok(())
            }

            let daily_close: f64 = closes
                .get(&date_str)
                .unwrap_or_else(|| panic!("/!\\ {date} should've had a price /!\\"))
                .to_owned();

            println!("{:?} - Processing {date} ({} blocks)...", Local::now(), blocks.len());

            let block_metadatas_len = block_metadatas_per_day.len();
            let len_diff = block_metadatas_len as i32 - STH_MAX_AGE as i32;
            if len_diff >= 0 {
                if len_diff > 0 {
                    panic!(
                        "There ({block_metadatas_len}) shouldn't be more than {STH_MAX_AGE} dayblocks"
                    );
                }

                block_metadatas_per_day
                    .pop_front()
                    .unwrap()
                    .iter()
                    .for_each(|block_metadata|
                        block_metadata
                            .outputs_by_txids
                            .borrow()
                            .keys()
                            .for_each(|txid| {
                                txid_to_block_metadata.remove(txid);
                            })
                    )
                ;
            }

            let block_metadatas = blocks.iter().enumerate().map(|(index, block)| {
                let height = first_block_index + index;
                let price = daily_close;

                let block_meta = Rc::new(ProcessingBlockMetaData {
                    height,
                    price,
                    outputs_by_txids: RefCell::new(FxHashMap::default()),
                });

                block.txdata.iter().for_each(|tx| {
                    let txid = tx.txid;

                    txid_to_block_metadata.insert(txid, Rc::clone(&block_meta));

                    {
                        block_meta.outputs_by_txids.borrow_mut().insert(txid, RefCell::new(
                            tx
                                .output
                                .iter()
                                .enumerate()
                                .map(|(usize, txout)| (usize as u32, sats_to_bitcoins(txout.value)))
                                .collect()
                            )
                        );
                    };

                    tx.input.iter().for_each(|txin| {
                        let outpoint = txin.previous_output;
                        let txid = outpoint.txid;
                        let vout = outpoint.vout;

                        let block_meta_opt = {
                            txid_to_block_metadata.get(&txid).map(Rc::clone)
                        };

                        if let Some(block_meta) = block_meta_opt {
                            let mut outputs_by_txids = block_meta
                                .outputs_by_txids
                                .borrow_mut();

                            let remove = {
                                let mut dataset =
                                    outputs_by_txids
                                    .get(&txid)
                                    .unwrap()
                                    .borrow_mut();

                                dataset.remove(&vout);

                                dataset.is_empty()
                            };

                            if remove {
                                outputs_by_txids.remove(&txid);
                                txid_to_block_metadata.remove(&txid);
                            }
                        }
                    });
                });

                Rc::clone(&block_meta)
            }).collect::<Vec<_>>();
            block_metadatas_per_day.push_back(block_metadatas);

            if latest_date.map_or(false, |latest_date| date < latest_date) {
                println!("{:?} - Not saving {date}...", Local::now());

                return Ok(())
            }

            println!("{:?} - Computing {date}...", Local::now());

            let mut total_list: Vec<f64> = vec![];
            let mut in_profit_list: Vec<f64> = vec![];

            block_metadatas_per_day
                .iter()
                .flatten()
                .for_each(
                    |block_meta| {
                        let values = block_meta
                            .outputs_by_txids
                            .borrow()
                            .values()
                            .map(|map|
                                map
                                    .borrow()
                                    .values()
                                    .sum::<f64>()
                            )
                            .sum();

                        total_list.push(values);

                        // TO_FIX: After hourly
                        if previous_daily_close > daily_close {
                            if block_meta.price < daily_close {
                                in_profit_list.push(values);
                            }
                        } else if block_meta.price <= daily_close {
                            in_profit_list.push(values);
                        }
                    }
                );

            previous_daily_close = daily_close;

            let to_save = SavingBlockMetaData {
                total: total_list.iter().sum(),
                in_profit: in_profit_list.iter().sum()
            };

            println!("{:?} - Saving {date}...", Local::now());

            insert_to_saved_map(
                &path_sth,
                date_str,
                to_save
            )?;

            Ok(())
        })?;

    Ok(())
}

fn sats_to_bitcoins(quantity: u64) -> f64 {
    quantity as f64 / 100_000_000.0
}
