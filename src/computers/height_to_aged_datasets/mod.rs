use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::{offset::Local, Datelike, Days, NaiveDate};
use itertools::Itertools;

use std::{cell::RefCell, rc::Rc};

use crate::{
    computers::height_to_aged_datasets::structs::{DateData, HeightToAgedDatasets},
    structs::{DateMap, TxidMap, NUMBER_OF_UNSAFE_BLOCKS},
    utils::{create_group_blocks_by_day_closure, ftransaction_to_outputs, timestamp_to_naive_date},
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
    let aged_datasets = HeightToAgedDatasets::import()?;

    let mut block_datas_per_day = BlockDatasPerDay::import(height_to_date)?;

    let snapshot_start_height = block_datas_per_day
        .iter()
        .map(|date_data| date_data.date)
        .max()
        .and_then(|date| date.checked_add_days(Days::new(1)))
        .and_then(|date| {
            date_to_first_block.get(&date).map(|snapshot_start_height| {
                if aged_datasets.get_min_last_height().unwrap_or(0) < snapshot_start_height {
                    panic!("snapshot_start_height > last_saved_height")
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
                blocks: RefCell::new(vec![]),
            });

            let block_data_list = &block_datas_per_day.last().unwrap().blocks;

            blocks.iter().enumerate().for_each(|(index, block)| {
                let height = current_height + index;

                let price = height_to_price
                    .get(height)
                    .unwrap_or_else(|| panic!("Expect {height} to have a price"))
                    .to_owned();

                let block_data = Rc::new(BlockData::new(price));

                block.txdata.iter().for_each(|tx| {
                    let txid = tx.txid;
                    txid_counter += 1;
                    let txid_index = txid_counter - 1;
                    txid_to_index.insert(txid, txid_index);

                    txid_index_to_block_data.insert(txid_index, Rc::clone(&block_data));

                    // Before `input` as some inputs can be used as later outputs
                    block_data
                        .txid_index_to_outputs
                        .borrow_mut()
                        .insert(txid_index, RefCell::new(ftransaction_to_outputs(tx)));

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
                            txid_index_to_block_data.get(&txid_index).map(Rc::clone)
                        {
                            let mut txid_index_to_outputs =
                                block_data.txid_index_to_outputs.borrow_mut();

                            let empty_outputs = {
                                let mut outputs = txid_index_to_outputs
                                    .get(&txid_index)
                                    .expect("txid_index to be in txid_index_to_outputs map")
                                    .borrow_mut();

                                outputs.remove(&vout);

                                outputs.is_empty()
                            };

                            if empty_outputs {
                                txid_index_to_outputs.remove(&txid_index);
                                txid_index_to_block_data.remove(&txid_index);
                                txid_to_index.remove(&txid);
                            }
                        }
                    });
                });

                block_data_list.borrow_mut().push(Rc::clone(&block_data));

                aged_datasets.insert(&block_datas_per_day, height, block_data.price);
            });

            println!("{:?} - Saving {date}...", Local::now());

            current_height += blocks_len;

            if (date.day() == 1 || date.day() == 14 || block_count - 1000 < current_height)
                && current_height < block_count - NUMBER_OF_UNSAFE_BLOCKS
            {
                block_datas_per_day.iter().for_each(|date_data| {
                    date_data
                        .blocks
                        .borrow_mut()
                        .retain(|block_data| block_data.txid_index_to_outputs.borrow().len() != 0)
                });

                aged_datasets.export()?;

                block_datas_per_day.export()?;

                txid_to_index.export()?;
            }

            Ok(())
        })?;

    aged_datasets.export()?;

    Ok(aged_datasets)
}
