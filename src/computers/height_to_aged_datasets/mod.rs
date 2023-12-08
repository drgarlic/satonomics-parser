use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::{offset::Local, Datelike, Days, NaiveDate};
use itertools::Itertools;

use std::sync::{Arc, RwLock};

use crate::{
    computers::height_to_aged_datasets::structs::{DateData, HeightToAgedDatasets},
    structs::{DateMap, TxidMap, NUMBER_OF_UNSAFE_BLOCKS},
    utils::{create_group_blocks_by_day_closure, timestamp_to_naive_date},
};

pub mod structs;

pub use self::structs::HeightToAgedDataset;
use self::structs::{BlockData, BlockDatasPerDay, TxidIndexToBlockData};

pub fn compute_height_to_aged_datasets(
    db: &BitcoinDB,
    block_count: usize,
    height_to_price: &[f32],
    height_to_date: &[NaiveDate],
    date_to_first_block: &DateMap<usize>,
) -> color_eyre::Result<HeightToAgedDatasets> {
    let aged_datasets = HeightToAgedDatasets::new()?;

    // 229mb

    let mut block_datas_per_day = BlockDatasPerDay::import(height_to_date)?;

    // 18.65gb
    // println!("loop");
    // loop {}

    let snapshot_start_height = block_datas_per_day
        .iter()
        .map(|date_data| date_data.date)
        .max()
        .and_then(|date| date.checked_add_days(Days::new(1)))
        .and_then(|date| {
            date_to_first_block.get(&date).map(|snapshot_start_height| {
                let min_last_height = aged_datasets.get_min_last_height();

                if min_last_height.unwrap_or(0) < snapshot_start_height - 1 {
                    panic!("snapshot_start_height {snapshot_start_height} > last_saved_height {min_last_height:?}");
                }

                snapshot_start_height
            })
        });

    let mut txid_index_to_block_data = TxidIndexToBlockData::from(&block_datas_per_day);

    let txid_to_index = TxidMap::import("height_to_aged__txid_to_index")?;

    let mut txid_counter = txid_to_index
        .borrow_map()
        .values()
        .max()
        .map(|index| *index + 1)
        .unwrap_or(0)
        .to_owned();

    let start_height = snapshot_start_height.unwrap_or(0);

    let mut current_height = start_height;

    db.iter_block::<FBlock>(start_height, block_count)
        .batching(create_group_blocks_by_day_closure())
        .try_for_each(|blocks| -> color_eyre::Result<()> {
            let date = timestamp_to_naive_date(blocks.first().unwrap().header.time);

            let blocks_len = blocks.len();

            println!(
                "{:?} - Processing {date} ({} blocks)...",
                Local::now(),
                blocks_len + 1
            );

            block_datas_per_day.push(DateData {
                date,
                blocks: RwLock::new(vec![]),
            });

            let block_data_list = &block_datas_per_day.last().unwrap().blocks;

            blocks.into_iter().enumerate().for_each(|(index, block)| {
                let height = current_height + index;

                let price = height_to_price
                    .get(height)
                    .unwrap_or_else(|| panic!("Expect {height} to have a price"))
                    .to_owned();

                let block_data = Arc::new(BlockData::new(price));

                block.txdata.into_iter().for_each(|tx| {
                    let txid = tx.txid;

                    txid_counter += 1;
                    let txid_index = txid_counter - 1;
                    txid_to_index.insert(txid, txid_index);

                    txid_index_to_block_data.insert(txid_index, Arc::clone(&block_data));

                    // Before `input` as some inputs can be used as later outputs
                    block_data.insert_outputs(txid_index, &tx);

                    tx.input.iter().for_each(|txin| {
                        let outpoint = txin.previous_output;
                        let txid = outpoint.txid;
                        let vout = outpoint.vout;

                        let txid_index = txid_to_index
                            .borrow_map()
                            .get(&txid)
                            .expect("Txid to be in txid_to_index map")
                            .to_owned();

                        if let Some(block_data) =
                            txid_index_to_block_data.get(&txid_index).map(Arc::clone)
                        {
                            let mut txid_index_to_outputs =
                                block_data.txid_index_to_outputs.write().unwrap();

                            let empty_outputs = {
                                let outputs = txid_index_to_outputs
                                    .get_mut(&txid_index)
                                    .expect("txid_index to be in txid_index_to_outputs map")
                                    .get_mut()
                                    .unwrap();

                                *block_data.amount.write().unwrap() -=
                                    outputs.remove(&vout).unwrap();

                                let empty = outputs.is_empty();

                                if !empty {
                                    outputs.shrink_to_fit();
                                }

                                empty
                            };

                            if empty_outputs {
                                txid_index_to_outputs.remove(&txid_index);

                                txid_index_to_block_data.remove(&txid_index);

                                txid_to_index.borrow_mut_map().remove(&txid);
                            }
                        }
                    });
                });

                block_data_list
                    .write()
                    .unwrap()
                    .push(Arc::clone(&block_data));

                aged_datasets.insert(&block_datas_per_day, height, block_data.price);
            });

            current_height += blocks_len;

            if (date.day() == 1 || date.day() == 14 || block_count - 1000 < current_height)
                && current_height < block_count - NUMBER_OF_UNSAFE_BLOCKS
            {
                shrink_all(
                    &mut block_datas_per_day,
                    &txid_to_index,
                    &mut txid_index_to_block_data,
                );

                export_all(&date, &aged_datasets, &block_datas_per_day, &txid_to_index)?;
            }

            Ok(())
        })?;

    aged_datasets.export()?;

    Ok(aged_datasets)
}

fn shrink_all(
    block_datas_per_day: &mut BlockDatasPerDay,
    txid_to_index: &TxidMap<usize>,
    txid_index_to_block_data: &mut TxidIndexToBlockData,
) {
    block_datas_per_day.iter().for_each(|date_data| {
        let mut blocks = date_data.blocks.write().unwrap();

        blocks.retain(|block_data| {
            let mut txid_index_to_outputs = block_data.txid_index_to_outputs.write().unwrap();

            let empty = txid_index_to_outputs.is_empty();

            if !empty {
                txid_index_to_outputs.shrink_to_fit();

                true
            } else {
                false
            }
        });

        blocks.shrink_to_fit();
    });

    block_datas_per_day.shrink_to_fit();

    txid_to_index.borrow_mut_map().shrink_to_fit();

    txid_index_to_block_data.shrink_to_fit();
}

fn export_all(
    date: &NaiveDate,
    aged_datasets: &HeightToAgedDatasets,
    block_datas_per_day: &BlockDatasPerDay,
    txid_to_index: &TxidMap<usize>,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving {date}... (Don't close !!)", Local::now());

    aged_datasets.export()?;

    block_datas_per_day.export()?;

    txid_to_index.export()?;

    Ok(())
}
