use std::{cmp::Ordering, time::Instant};

use chrono::{offset::Local, Datelike};

use crate::{
    bitcoin::{BitcoinDB, NUMBER_OF_UNSAFE_BLOCKS},
    databases::Databases,
    datasets::{AllDatasets, AnyDateDatasets, ProcessedDateData},
    export_all::{export_all, ExportedData},
    min_height::min_height,
    parse_block::{parse_block, ParseData},
    states::States,
    structs::DateData,
    utils::timestamp_to_naive_date,
};

pub fn iter_blocks(bitcoin_db: &BitcoinDB, block_count: usize) -> color_eyre::Result<()> {
    println!("{:?} - Starting aged", Local::now());

    let mut datasets = AllDatasets::import()?;

    println!("{:?} - Imported datasets", Local::now());

    let mut databases = Databases::default();
    let mut states = States::import().unwrap_or_default();

    let mut height = min_height(&mut states, &databases, &datasets);

    println!("{:?} - Starting parsing at height: {height}", Local::now());

    let mut block_iter = bitcoin_db.iter_block(height, block_count);

    let mut parsing = true;
    let mut saved_block_opt = None;
    let mut blocks_date_opt = None;

    while parsing {
        let time = Instant::now();

        'days: loop {
            let mut block_count = 0;

            blocks_date_opt.take();

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
                    let timestamp = current_block.header.time;

                    let current_block_date = timestamp_to_naive_date(timestamp);

                    // Always run for the first block of the loop
                    if blocks_date_opt.is_none() {
                        blocks_date_opt.replace(current_block_date);

                        if states
                            .date_data_vec
                            .last()
                            .map(|date_data| *date_data.date < current_block_date)
                            .unwrap_or(true)
                        {
                            states
                                .date_data_vec
                                .push(DateData::new(current_block_date, vec![]));
                        }

                        println!("{:?} - Processing {current_block_date}...", Local::now());
                    }

                    let blocks_date = blocks_date_opt.unwrap();

                    match blocks_date.cmp(&current_block_date) {
                        Ordering::Equal | Ordering::Greater => {
                            let block_index = block_count;

                            block_count += 1;

                            parse_block(ParseData {
                                bitcoin_db,
                                block: current_block,
                                block_index,
                                databases: &mut databases,
                                datasets: &mut datasets,
                                date: current_block_date,
                                height: height + block_index,
                                timestamp,
                                states: &mut states,
                            });
                        }
                        Ordering::Less => {
                            datasets.date.insert(ProcessedDateData {
                                block_count,
                                first_height: height,
                                height: height + block_count,
                                date: blocks_date,
                            });

                            saved_block_opt.replace(current_block);

                            height += block_count;

                            if blocks_date.day() == 1
                                || (block_count - NUMBER_OF_UNSAFE_BLOCKS * 10) < height
                            {
                                break 'days;
                            } else {
                                break 'blocks;
                            }
                        }
                    }
                } else {
                    height += block_count;

                    parsing = false;

                    break 'days;
                }
            }
        }

        let last_height = height - 1;

        println!(
            "Parsing month took {} seconds (last height: {last_height})\n",
            time.elapsed().as_secs_f32(),
        );

        export_all(ExportedData {
            block_count,
            databases: &mut databases,
            datasets: &datasets,
            date: blocks_date_opt.unwrap(),
            height: last_height,
            states: &states,
        })?;
    }

    datasets.export()?;

    Ok(())
}
