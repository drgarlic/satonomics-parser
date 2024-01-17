use std::{cmp::Ordering, time::Instant};

use bitcoin::Block;
use chrono::{offset::Local, Datelike, NaiveDate};

use crate::{
    bitcoin::{BitcoinDB, NUMBER_OF_UNSAFE_BLOCKS},
    structs::DateMap,
    utils::timestamp_to_naive_date,
};

mod databases;
mod datasets;
mod export_all;
mod min_height;
mod parse_block;
mod states;
mod structs;

use databases::*;
use datasets::*;
use export_all::*;
use export_all::*;
use min_height::*;
use parse_block::*;
use parse_block::*;
use states::*;
use structs::*;

pub fn compute_utxo_based_datasets(
    bitcoin_db: &BitcoinDB,
    block_count: usize,
    height_to_price: &[f32],
    date_to_first_block: &DateMap<usize>,
) -> color_eyre::Result<Datasets> {
    println!("{:?} - Starting aged", Local::now());

    let mut datasets = Datasets::new()?;

    println!("{:?} - Imported datasets", Local::now());

    let mut databases = Databases::default();
    let mut states = States::import().unwrap_or_default();

    let mut height = min_height(&mut states, &datasets, date_to_first_block);

    println!("{:?} - Starting parsing", Local::now());

    let mut block_iter = bitcoin_db.iter_block(height, block_count);

    let mut parsing = true;
    let mut saved_block_opt: Option<Block> = None;
    let mut last_date_opt: Option<NaiveDate> = None;

    while parsing {
        let time = Instant::now();

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

                        states.date_data_vec.push(DateData::new(date, vec![]));

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
                                bitcoin_db,
                                block: current_block,
                                block_index,
                                databases: &mut databases,
                                datasets: &mut datasets,
                                date,
                                height: height + block_index,
                                height_to_price,
                                timestamp,
                                states: &mut states,
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

        println!(
            "Parsing month took {} seconds\n",
            time.elapsed().as_secs_f32()
        );

        export_all(ExportedData {
            databases: &mut databases,
            block_count,
            datasets: &datasets,
            height,
            states: &states,
        })?;
    }

    datasets.export()?;

    Ok(datasets)
}
