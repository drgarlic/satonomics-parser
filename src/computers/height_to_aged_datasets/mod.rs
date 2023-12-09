use bitcoin_explorer::{BitcoinDB, FBlock, FTransaction};
use chrono::{offset::Local, Datelike, Days, NaiveDate};
use itertools::Itertools;

use std::{
    ops::ControlFlow,
    sync::{Arc, RwLock},
};

use crate::{
    computers::height_to_aged_datasets::structs::{DateData, HeightToAgedDatasets, Txtuple},
    structs::{DateMap, NUMBER_OF_UNSAFE_BLOCKS},
    utils::{
        convert_sats_to_bitcoins, create_group_blocks_by_day_closure, timestamp_to_naive_date,
    },
};

pub mod structs;

pub use self::structs::HeightToAgedDataset;
use self::structs::{
    BlockData, BlockDatasPerDay, TxidIndexToBlockData, TxidToTxtuple, TxoutIndexToValue,
};

pub fn compute_height_to_aged_datasets(
    db: &BitcoinDB,
    block_count: usize,
    height_to_price: &[f32],
    height_to_date: &[NaiveDate],
    date_to_first_block: &DateMap<usize>,
) -> color_eyre::Result<HeightToAgedDatasets> {
    let aged_datasets = HeightToAgedDatasets::import()?;

    let mut block_datas_per_day = BlockDatasPerDay::import(height_to_date)?;

    let mut txid_index_to_block_data = TxidIndexToBlockData::import(&block_datas_per_day)?;

    // Ram usage: 80% of serialized file (ex: ~6.6 > ~5.3)
    let txid_to_txtuple = TxidToTxtuple::import()?;

    let mut txout_index_to_value = TxoutIndexToValue::import()?;

    let mut txout_index_counter = txout_index_to_value
        .keys()
        .max()
        .map(|index| *index + 1)
        .unwrap_or(0)
        .to_owned();

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

                let block_data = Arc::new(BlockData::new(height, price));

                block_data_list
                    .write()
                    .unwrap()
                    .push(Arc::clone(&block_data));

                block.txdata.into_iter().for_each(|tx| {
                    let txid = tx.txid;

                    let txid_index = txout_index_counter;

                    txout_index_counter += tx.output.len();

                    // --
                    // outputs
                    // ---

                    // 0 sats outputs are possible and allowed !
                    // https://mempool.space/tx/2f2442f68e38b980a6c4cec21e71851b0d8a5847d85208331a27321a9967bbd6
                    // https://bitcoin.stackexchange.com/questions/104937/transaction-outputs-with-value-0
                    let mut non_zero_outputs_len = 0;

                    // Before `input` as some inputs can be used as later outputs
                    tx.output
                        .into_iter()
                        .enumerate()
                        .map(|(vout, txout)| (vout, txout.value))
                        .filter(|(_, value)| value > &0)
                        .for_each(|(vout, value)| {
                            non_zero_outputs_len += 1;

                            let value = convert_sats_to_bitcoins(value);

                            *block_data.amount.write().unwrap() += value;

                            let txout_index = txid_index + vout;

                            // if txid.to_string() == "2f2442f68e38b980a6c4cec21e71851b0d8a5847d85208331a27321a9967bbd6" || txout_index == 144533 {
                            //     dbg!((txid_index, vout, value));
                            // }

                            txout_index_to_value.insert(txout_index, value);
                        });

                    if non_zero_outputs_len != 0 {
                        txid_to_txtuple
                            .insert(txid, Txtuple::new(txid_index, non_zero_outputs_len as u32));

                        txid_index_to_block_data.insert(txid_index, Arc::clone(&block_data));

                        *block_data.outputs_len.write().unwrap() += non_zero_outputs_len as u32;
                    }

                    // ---
                    // inputs
                    // ---

                    tx.input.into_iter().try_for_each(|txin| {
                        let outpoint = txin.previous_output;
                        let input_txid = outpoint.txid;
                        let input_vout = outpoint.vout;

                        let txid_index_to_remove = {
                            let mut txid_to_txtuple = txid_to_txtuple.borrow_mut_map();

                            let txtuple = txid_to_txtuple.get_mut(&input_txid);

                            let get_value_from_db = || {
                                db.get_transaction::<FTransaction>(&input_txid)
                                    .unwrap()
                                    .output
                                    .get(input_vout as usize)
                                    .unwrap()
                                    .value
                            };

                            if txtuple.is_none() {
                                if get_value_from_db() == 0 {
                                    return ControlFlow::Continue::<()>(());
                                } else {
                                    dbg!((txid, input_txid, txid_index, input_vout));
                                    panic!("Txid to be in txid_to_txtuple");
                                }
                            }

                            let txtuple = txtuple.unwrap();

                            let value = txout_index_to_value
                                .remove(&(txtuple.txid_index + input_vout as usize));

                            if value.is_none() {
                                if get_value_from_db() == 0 {
                                    return ControlFlow::Continue::<()>(());
                                } else {
                                    dbg!((
                                        txid, input_txid, txid_index, &txtuple, input_vout, value
                                    ));
                                    panic!("Txout index to be in txout_index_to_value");
                                }
                            }

                            let value = value.unwrap();

                            if let Some(previous_block_data) =
                                txid_index_to_block_data.get(&txtuple.txid_index)
                            {
                                *previous_block_data.amount.write().unwrap() -= value;

                                *previous_block_data.outputs_len.write().unwrap() -= 1;
                            }

                            txtuple.outputs_len -= 1;

                            if txtuple.outputs_len == 0 {
                                Some(txtuple.txid_index)
                            } else {
                                None
                            }
                        };

                        if let Some(txid_index_to_remove) = txid_index_to_remove {
                            txid_to_txtuple.borrow_mut_map().remove(&input_txid);

                            txid_index_to_block_data.remove(&txid_index_to_remove);
                        }

                        ControlFlow::Continue(())
                    });
                });

                aged_datasets.insert(&block_datas_per_day, height, block_data.price);
            });

            current_height += blocks_len;

            if (date.day() == 1 || date.day() == 14 || block_count - 1000 < current_height)
                && current_height < block_count - NUMBER_OF_UNSAFE_BLOCKS
            {
                retain_all(&mut block_datas_per_day);

                export_all(
                    &date,
                    &aged_datasets,
                    &block_datas_per_day,
                    &txid_to_txtuple,
                    &txout_index_to_value,
                    &txid_index_to_block_data,
                )?;
            }

            Ok(())
        })?;

    aged_datasets.export()?;

    Ok(aged_datasets)
}

fn export_all(
    date: &NaiveDate,
    aged_datasets: &HeightToAgedDatasets,
    block_datas_per_day: &BlockDatasPerDay,
    txid_to_txtuple: &TxidToTxtuple,
    txout_index_to_value: &TxoutIndexToValue,
    txid_index_to_block_data: &TxidIndexToBlockData,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving {date}... (Don't close !!)", Local::now());

    aged_datasets.export()?;

    block_datas_per_day.export()?;

    txid_to_txtuple.export()?;

    txout_index_to_value.export()?;

    txid_index_to_block_data.export()?;

    Ok(())
}

fn retain_all(block_datas_per_day: &mut BlockDatasPerDay) {
    block_datas_per_day.iter().for_each(|date_data| {
        let mut blocks = date_data.blocks.write().unwrap();

        blocks.retain(|block_data| block_data.outputs_len.read().unwrap().to_owned() != 0);
    });
}
