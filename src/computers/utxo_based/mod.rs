use std::{cmp::Ordering, thread};

use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::{offset::Local, Datelike, NaiveDate};

use crate::{
    computers::{
        export::{export_all, ExportData},
        process::{process_block, ProcessData},
    },
    structs::{DateMap, HeightDatasets, NUMBER_OF_UNSAFE_BLOCKS},
    utils::timestamp_to_naive_date,
};

pub mod export;
pub mod process;
pub mod structs;

use structs::*;

pub fn compute_utxo_based_datasets(
    bitcoin_db: &BitcoinDB,
    block_count: usize,
    height_to_price: &[f32],
    date_to_first_block: &DateMap<usize>,
) -> color_eyre::Result<UtxoDatasets> {
    println!("{:?} - Starting aged", Local::now());

    let datasets = UtxoDatasets::new()?;

    println!("{:?} - Imported datasets", Local::now());

    let InitiatedParsers {
        mut address_counter,
        mut address_index_to_address_data,
        mut date_data_vec,
        mut txid_to_tx_index,
        mut tx_counter,
        mut tx_index_to_tx_data,
        mut txout_index_to_txout_data,
        mut height,
    } = InitiatedParsers::init(&datasets, date_to_first_block);

    println!("{:?} - Starting parsing", Local::now());

    let mut block_iter = bitcoin_db.iter_block::<FBlock>(height, block_count);

    let mut parsing = true;
    let mut saved_block_opt: Option<FBlock> = None;
    let mut last_date_opt: Option<NaiveDate> = None;

    while parsing {
        let mut address_index_to_empty_address_data = AddressIndexToEmptyAddressData::open(height)?;
        let mut address_to_address_index = AddressToAddressIndex::open(height)?;

        'days: loop {
            let mut block_len = 0;

            'blocks: loop {
                let current_block = {
                    let saved_block = saved_block_opt.take();

                    if saved_block.is_some() {
                        saved_block
                    } else {
                        block_iter.next()
                    }
                };

                if let Some(current_block) = current_block {
                    if last_date_opt.is_none() {
                        let date = timestamp_to_naive_date(current_block.header.time);

                        last_date_opt.replace(date);

                        date_data_vec.push(DateData::new(date, vec![]));

                        println!("{:?} - Processing {date}...", Local::now());
                    }

                    let last_date = last_date_opt.unwrap();

                    let block_date = timestamp_to_naive_date(current_block.header.time);

                    match last_date.cmp(&block_date) {
                        Ordering::Equal | Ordering::Greater => {
                            block_len += 1;

                            let block_index = block_len - 1;

                            process_block(ProcessData {
                                bitcoin_db,
                                block: current_block,
                                block_index,
                                block_height: height + block_index,
                                date: block_date,
                                date_data_vec: &mut date_data_vec,
                                height_to_price,
                                address_to_address_index: &mut address_to_address_index,
                                // address_index_to_address: &mut address_index_to_address,
                                address_index_to_address_data: &mut address_index_to_address_data,
                                address_index_to_empty_address_data:
                                    &mut address_index_to_empty_address_data,
                                // empty_address_to_address_index: &mut empty_address_to_address_index,
                                txid_to_tx_index: &mut txid_to_tx_index,
                                tx_index_to_tx_data: &mut tx_index_to_tx_data,
                                txout_index_to_txout_data: &mut txout_index_to_txout_data,
                                tx_counter: &mut tx_counter,
                                address_counter: &mut address_counter,
                            });
                        }
                        Ordering::Less => {
                            saved_block_opt.replace(current_block);
                            last_date_opt.take();

                            height += block_len;

                            if last_date.day() == 1
                                || (block_count - NUMBER_OF_UNSAFE_BLOCKS * 10) < height
                            {
                                break 'days;
                            } else {
                                break 'blocks;
                            }
                        }
                    }
                } else {
                    height += block_len;

                    parsing = false;

                    break 'days;
                }
            }
        }

        parsing = false;

        export_all(ExportData {
            address_counter: &address_counter,
            address_index_to_address_data: &address_index_to_address_data,
            address_index_to_empty_address_data,
            address_to_address_index,
            datasets: &datasets,
            date_data_vec: &date_data_vec,
            height,
            tx_index_to_tx_data: &tx_index_to_tx_data,
            txid_to_tx_index: &txid_to_tx_index,
            txout_index_to_txout_data: &txout_index_to_txout_data,
        })?;
    }

    datasets.export()?;

    Ok(datasets)
}
