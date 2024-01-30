use std::time::Instant;

use chrono::{offset::Local, Datelike};

use crate::{
    bitcoin::{check_if_height_safe, BitcoinDB},
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
    let mut next_block_opt = None;
    let mut blocks_loop_date = None;

    while parsing {
        let time = Instant::now();

        'days: loop {
            let mut blocks_loop_i = 0;

            blocks_loop_date.take();

            'blocks: loop {
                let current_block_opt = next_block_opt.take().or_else(|| block_iter.next());

                next_block_opt = block_iter.next();

                if let Some(current_block) = current_block_opt {
                    let timestamp = current_block.header.time;

                    let current_block_date = timestamp_to_naive_date(timestamp);
                    let current_block_height = height + blocks_loop_i;

                    // Always run for the first block of the loop
                    if blocks_loop_date.is_none() {
                        blocks_loop_date.replace(current_block_date);

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

                        println!(
                            "{:?} - Processing {current_block_date} (height: {height})...",
                            Local::now()
                        );
                    }

                    let blocks_loop_date = blocks_loop_date.unwrap();

                    if current_block_date > blocks_loop_date {
                        panic!("current block should always have the same date as the current blocks loop");
                    }

                    let is_date_last_block = next_block_opt.is_none()
                        || current_block_date
                            < timestamp_to_naive_date(next_block_opt.as_ref().unwrap().header.time);

                    parse_block(ParseData {
                        bitcoin_db,
                        block: current_block,
                        block_index: blocks_loop_i,
                        databases: &mut databases,
                        datasets: &mut datasets,
                        date: current_block_date,
                        height: current_block_height,
                        is_date_last_block,
                        states: &mut states,
                        timestamp,
                    });

                    blocks_loop_i += 1;

                    if is_date_last_block {
                        datasets.date.insert(ProcessedDateData {
                            block_count,
                            first_height: height,
                            height: current_block_height,
                            date: blocks_loop_date,
                        });

                        height += blocks_loop_i;

                        if blocks_loop_date.day() == 1 || check_if_height_safe(height, block_count)
                        {
                            break 'days;
                        } else {
                            break 'blocks;
                        }
                    }
                } else {
                    height += blocks_loop_i;

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
            date: blocks_loop_date.unwrap(),
            height: last_height,
            states: &states,
        })?;
    }

    datasets.export()?;

    Ok(())
}
