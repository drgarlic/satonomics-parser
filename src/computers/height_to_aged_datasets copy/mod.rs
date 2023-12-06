use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::{offset::Local, Datelike};
use itertools::Itertools;

use std::{cell::RefCell, rc::Rc};

use crate::{
    computers::height_to_aged_datasets::structs::{DateData, HeightToAgedDatasets},
    structs::{DateMap, TxidMap, TxidToOutputs},
    utils::{create_group_blocks_by_day_closure, string_to_naive_date, timestamp_to_naive_date},
};

pub mod structs;

pub use self::structs::HeightToAgedDataset;
use self::structs::{BlockData, BlockDatasPerDay, TxidToBlockData};

pub fn compute_height_to_aged_datasets(
    db: &BitcoinDB,
    block_count: usize,
    height_to_price: &[f32],
    date_to_first_block: &DateMap<usize>,
    date_to_last_block: &DateMap<usize>,
) -> color_eyre::Result<HeightToAgedDatasets> {
    let aged_datasets = HeightToAgedDatasets::import()?;

    let mut start_date = None;

    let min_unsafe_height = aged_datasets.get_min_unsafe_height();
    println!("Min unsafe height: {min_unsafe_height:?}");

    let min_unsafe_date = min_unsafe_height.and_then(|min_unsafe_height| {
        date_to_last_block
            .to_sorted_vec()
            .iter()
            .find(|(_, last_block)| *last_block >= min_unsafe_height)
            .map(|(date, _)| string_to_naive_date(date))
    });

    let mut block_datas_per_day: BlockDatasPerDay = BlockDatasPerDay::new();

    let txid_to_block_data: TxidToBlockData = TxidMap::new(None);

    let start_height = 0;
    // min_unsafe_date
    // .and_then(|date| {
    //     let date = date
    //         .checked_sub_days(Days::new(STH_MAX_AGE as u64))
    //         .unwrap();

    //     start_date.replace(date);

    //     println!("Starting date: {date:?}");

    //     date_to_first_block.get(&date)
    // })
    // .unwrap_or(0);

    let mut current_height = start_height;

    db.iter_block::<FBlock>(start_height, block_count)
        .batching(create_group_blocks_by_day_closure())
        .try_for_each(|blocks| -> color_eyre::Result<()> {
            let date = timestamp_to_naive_date(blocks.first().unwrap().header.time);

            let blocks_len = blocks.len();

            if start_date
                .map(|start_date| date < start_date)
                .unwrap_or(false)
            {
                println!(
                    "{:?} - Skipping {date} ({} blocks)...",
                    Local::now(),
                    blocks_len
                );

                return Ok(());
            }

            println!(
                "{:?} - Processing {date} ({} blocks)...",
                Local::now(),
                blocks_len + 1
            );

            block_datas_per_day.push(DateData {
                date,
                blocks: RefCell::new(vec![]),
            });

            let block_data_list = &block_datas_per_day.last().unwrap().blocks;

            blocks.iter().enumerate().for_each(|(index, block)| {
                let height = current_height + index;

                let price = height_to_price
                    .get(height)
                    .unwrap_or_else(|| panic!("Expect {height} to have a price"))
                    .to_owned();

                let block_data = Rc::new(BlockData {
                    price,
                    txid_to_outputs: TxidToOutputs::new(None),
                });

                block.txdata.iter().for_each(|tx| {
                    let txid = tx.txid;

                    txid_to_block_data.insert(txid.to_owned(), Rc::clone(&block_data));

                    block_data.txid_to_outputs.insert(tx);

                    tx.input.iter().for_each(|txin| {
                        let outpoint = txin.previous_output;
                        let txid = outpoint.txid;
                        let vout = outpoint.vout;

                        let block_data_opt =
                            { txid_to_block_data.borrow_map().get(&txid).map(Rc::clone) };

                        if let Some(block_data) = block_data_opt {
                            let mut txid_to_outputs = block_data.txid_to_outputs.borrow_mut_map();

                            let remove = {
                                let mut dataset = txid_to_outputs.get(&txid).unwrap().borrow_mut();

                                dataset.remove(&vout);

                                dataset.is_empty()
                            };

                            if remove {
                                txid_to_outputs.remove(&txid);
                                txid_to_block_data.remove(&txid);
                            }
                        }
                    });
                });

                block_data_list.borrow_mut().push(Rc::clone(&block_data));

                aged_datasets.insert(&block_datas_per_day, height, block_data.price);
            });

            // if min_unsafe_date.map_or(false, |min_unsafe_date| date < min_unsafe_date) {
            //     println!("{:?} - Not saving {date}...", Local::now());

            //     return Ok(());
            // }

            println!("{:?} - Saving {date}...", Local::now());

            if date.day() == 1 {
                block_datas_per_day.snapshot()?;
            }

            current_height += blocks_len;

            Ok(())
        })?;

    aged_datasets.export()?;

    Ok(aged_datasets)
}

// fn remove_excess(block_datas_per_day: &mut BlockDatasPerDay, txid_to_block_data: &TxidToBlockData) {
//     let block_datas_len = block_datas_per_day.len();
//     let len_diff = block_datas_len as i32 - STH_MAX_AGE as i32;
//     if len_diff >= 0 {
//         if len_diff > 0 {
//             panic!("There ({block_datas_len}) shouldn't be more than {STH_MAX_AGE} dayblocks");
//         }

//         block_datas_per_day
//             .pop_back()
//             .unwrap()
//             .iter()
//             .for_each(|block_data| {
//                 block_data
//                     .txid_to_outputs
//                     .borrow_map()
//                     .keys()
//                     .for_each(|txid| {
//                         txid_to_block_data.remove(txid);
//                     })
//             });
//     }
// }
