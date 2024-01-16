use std::cmp::Ordering;

use bitcoin::Block;
use chrono::{offset::Local, Datelike, NaiveDate};

use crate::{
    bitcoin::{BitcoinDB, NUMBER_OF_UNSAFE_BLOCKS},
    structs::DateMap,
    utils::timestamp_to_naive_date,
};

mod export_all;
mod parse_block;
pub mod structs;

use export_all::*;
use parse_block::*;

pub use structs::*;

pub fn compute_utxo_based_datasets(
    bitcoin_db: &BitcoinDB,
    block_count: usize,
    height_to_price: &[f32],
    date_to_first_block: &DateMap<usize>,
) -> color_eyre::Result<Datasets> {
    println!("{:?} - Starting aged", Local::now());

    let mut datasets = Datasets::new()?;

    println!("{:?} - Imported datasets", Local::now());

    let InitiatedParsers {
        mut address_index_to_address_data,
        mut counters,
        mut date_data_vec,
        mut tx_index_to_tx_data,
        mut txout_index_to_txout_data,
        mut height,
    } = InitiatedParsers::init(&datasets, date_to_first_block);

    println!("{:?} - Starting parsing", Local::now());

    let mut block_iter = bitcoin_db.iter_block(height, block_count);

    let mut parsing = true;
    let mut saved_block_opt: Option<Block> = None;
    let mut last_date_opt: Option<NaiveDate> = None;

    let mut address_index_to_empty_address_data = AddressIndexToEmptyAddressData::default();
    let mut raw_address_to_address_index = RawAddressToAddressIndex::default();
    let mut txid_to_tx_index = TxidToTxIndex::default();

    while parsing {
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

                    let timestamp = current_block.header.time;

                    let date = timestamp_to_naive_date(timestamp);

                    match last_date.cmp(&date) {
                        Ordering::Equal | Ordering::Greater => {
                            block_len += 1;

                            let block_index = block_len - 1;

                            parse_block(ParseData {
                                address_index_to_address_data: &mut address_index_to_address_data,
                                address_index_to_empty_address_data:
                                    &mut address_index_to_empty_address_data,
                                bitcoin_db,
                                block: current_block,
                                block_index,
                                counters: &mut counters,
                                datasets: &mut datasets,
                                date,
                                date_data_vec: &mut date_data_vec,
                                height: height + block_index,
                                height_to_price,
                                raw_address_to_address_index: &mut raw_address_to_address_index,
                                timestamp,
                                tx_index_to_tx_data: &mut tx_index_to_tx_data,
                                txid_to_tx_index: &mut txid_to_tx_index,
                                txout_index_to_txout_data: &mut txout_index_to_txout_data,
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

        export_all(ExportedData {
            address_index_to_address_data: &address_index_to_address_data,
            address_index_to_empty_address_data: &mut address_index_to_empty_address_data,
            raw_address_to_address_index: &mut raw_address_to_address_index,
            block_count,
            counters: &counters,
            datasets: &datasets,
            date_data_vec: &date_data_vec,
            height,
            tx_index_to_tx_data: &tx_index_to_tx_data,
            txid_to_tx_index: &mut txid_to_tx_index,
            txout_index_to_txout_data: &txout_index_to_txout_data,
        })?;
    }

    datasets.export()?;

    Ok(datasets)
}
