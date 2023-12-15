use bitcoin_explorer::{BitcoinDB, FBlock, FTransaction};
use chrono::{offset::Local, Datelike, NaiveDate};
use itertools::Itertools;

use std::{collections::BTreeMap, ops::ControlFlow};

use crate::{
    structs::{DateMap, HeightDatasets, WTxid, NUMBER_OF_UNSAFE_BLOCKS},
    utils::{
        convert_sats_to_bitcoins, create_group_blocks_by_day_closure, timestamp_to_naive_date,
    },
};

pub mod export;
pub mod parse;

use export::*;
use parse::*;

pub fn compute_utxo_based_datasets(
    db: &BitcoinDB,
    block_count: usize,
    height_to_price: &[f32],
    height_to_date: &[NaiveDate],
    date_to_first_block: &DateMap<usize>,
) -> color_eyre::Result<UtxoDatasets> {
    println!("{:?} - Starting aged", Local::now());

    let datasets = UtxoDatasets::new()?;

    println!("{:?} - Imported datasets", Local::now());

    let InitiatedParsers {
        mut date_data_vec,
        mut txid_to_tx_data,
        mut txout_index_counter,
        mut txid_index_to_block_path,
        mut txout_index_to_txout_value,
        mut iter_height,
    } = InitiatedParsers::init(&datasets, height_to_date, date_to_first_block)?;

    println!("{:?} - Starting export", Local::now());

    txid_to_tx_data.export()?;
    println!("{:?} - Exported txid_to_tx_data", Local::now());

    txout_index_to_txout_value.export()?;
    println!("{:?} - Exported txout_index_to_txout_value", Local::now());

    txid_index_to_block_path.export()?;
    println!("{:?} - Exported txid_index_to_block_path", Local::now());

    panic!("!!!");

    println!("current_height {iter_height}");

    println!("{:?} - Starting parsing", Local::now());

    db.iter_block::<FBlock>(iter_height, block_count)
        .batching(create_group_blocks_by_day_closure())
        .try_for_each(|blocks| -> color_eyre::Result<()> {
            let date = timestamp_to_naive_date(blocks.first().unwrap().header.time);

            let blocks_len = blocks.len();

            println!(
                "{:?} - Processing {date} ({} blocks)...",
                Local::now(),
                blocks_len + 1
            );

            date_data_vec.push(DateData {
                date,
                blocks: vec![],
            });

            let date_index = date_data_vec.len() - 1;

            blocks
                .into_iter()
                .enumerate()
                .for_each(|(block_index, block)| {
                    let block_height = iter_height + block_index;

                    let price = height_to_price
                        .get(block_height)
                        .unwrap_or_else(|| panic!("Expect {block_height} to have a price"))
                        .to_owned();

                    date_data_vec
                        .last_mut()
                        .unwrap()
                        .blocks
                        .push(BlockData::new(block_height as u32, price));

                    let mut coinbase = 0.0;
                    let mut inputs_sum = 0.0;
                    let mut outputs_sum = 0.0;

                    let mut stxos = BTreeMap::new();

                    let mut coinblocks_destroyed = 0.0;
                    let mut coindays_destroyed = 0.0;

                    block
                        .txdata
                        .into_iter()
                        .enumerate()
                        .for_each(|(tx_index, tx)| {
                            let txid = tx.txid;
                            let wtxid = WTxid::wrap(txid);

                            let txid_index = txout_index_counter;

                            txout_index_counter += tx.output.len();

                            // --
                            // outputs
                            // ---

                            // 0 sats outputs are possible and allowed !
                            // https://mempool.space/tx/2f2442f68e38b980a6c4cec21e71851b0d8a5847d85208331a27321a9967bbd6
                            // https://bitcoin.stackexchange.com/questions/104937/transaction-outputs-with-value-0
                            let mut non_zero_outputs_len = 0;
                            let mut non_zero_amount = 0.0;

                            // Before `input` as some inputs can be used as later outputs
                            tx.output
                                .into_iter()
                                .enumerate()
                                .map(|(vout, txout)| (vout, txout.value))
                                .filter(|(_, value)| value > &0)
                                .for_each(|(vout, value)| {
                                    non_zero_outputs_len += 1;

                                    let txout_value = convert_sats_to_bitcoins(value);

                                    non_zero_amount += txout_value;

                                    let txout_index = txid_index + vout;

                                    txout_index_to_txout_value.insert(txout_index, txout_value);
                                });

                            if non_zero_outputs_len != 0 {
                                txid_to_tx_data
                                    .insert(wtxid, TxData::new(txid_index, non_zero_outputs_len));

                                txid_index_to_block_path
                                    .insert(txid_index, BlockPath::build(date_index, block_index));

                                let last_block = date_data_vec.last_mut_block();

                                last_block.amount += non_zero_amount;
                                last_block.outputs_len += non_zero_outputs_len;

                                if tx_index == 0 {
                                    coinbase = non_zero_amount;
                                } else {
                                    outputs_sum += non_zero_amount;
                                }
                            }

                            // ---
                            // inputs
                            // ---

                            tx.input.into_iter().try_for_each(|txin| {
                                let outpoint = txin.previous_output;
                                let input_txid = outpoint.txid;
                                let input_wtxid = WTxid::wrap(input_txid);
                                let input_vout = outpoint.vout;

                                let txid_index_to_remove = {
                                    let input_tx_data = txid_to_tx_data.get_mut(&input_wtxid);

                                    let get_value_from_db = || {
                                        db.get_transaction::<FTransaction>(&input_wtxid)
                                            .unwrap()
                                            .output
                                            .get(input_vout as usize)
                                            .unwrap()
                                            .value
                                    };

                                    if input_tx_data.is_none() {
                                        if get_value_from_db() == 0 {
                                            return ControlFlow::Continue::<()>(());
                                        } else {
                                            dbg!((txid, input_wtxid, txid_index, input_vout));
                                            panic!("Txid to be in txid_to_tx_data");
                                        }
                                    }

                                    let input_tx_data = input_tx_data.unwrap();

                                    let input_value = txout_index_to_txout_value
                                        .remove(&(input_tx_data.txid_index + input_vout as usize));

                                    if input_value.is_none() {
                                        if get_value_from_db() == 0 {
                                            return ControlFlow::Continue::<()>(());
                                        } else {
                                            dbg!((
                                                txid,
                                                input_wtxid,
                                                txid_index,
                                                &input_tx_data,
                                                input_vout,
                                                input_value
                                            ));
                                            panic!(
                                                "Txout index to be in txout_index_to_txout_value"
                                            );
                                        }
                                    }

                                    let input_value = input_value.unwrap();

                                    if let Some(input_block_path) =
                                        txid_index_to_block_path.get(&input_tx_data.txid_index)
                                    {
                                        let SplitBlockPath {
                                            date_index: input_date_index,
                                            block_index: input_block_index,
                                        } = input_block_path.split();

                                        let input_date_data = date_data_vec
                                            .get_mut(input_date_index)
                                            .unwrap_or_else(|| {
                                                dbg!(
                                                    block_height,
                                                    &input_wtxid,
                                                    input_block_path,
                                                    input_date_index
                                                );
                                                panic!()
                                            });

                                        let input_block_data = input_date_data
                                            .blocks
                                            .get_mut(input_block_index)
                                            .unwrap();

                                        input_block_data.outputs_len -= 1;

                                        input_block_data.amount -= input_value;

                                        inputs_sum += input_value;

                                        stxos.insert(
                                            *input_block_path,
                                            stxos.get(input_block_path).unwrap_or(&0.0)
                                                + input_value,
                                        );

                                        coinblocks_destroyed += (block_height
                                            - input_block_data.height as usize)
                                            as f64
                                            * input_block_data.amount;

                                        coindays_destroyed += date
                                            .signed_duration_since(input_date_data.date)
                                            .num_days()
                                            as f64
                                            * input_block_data.amount;
                                    }

                                    input_tx_data.outputs_len -= 1;

                                    if input_tx_data.outputs_len == 0 {
                                        Some(input_tx_data.txid_index)
                                    } else {
                                        None
                                    }
                                };

                                if let Some(txid_index_to_remove) = txid_index_to_remove {
                                    txid_to_tx_data.remove(&input_wtxid);

                                    txid_index_to_block_path.remove(&txid_index_to_remove);
                                }

                                ControlFlow::Continue(())
                            });
                        });

                    let fees = inputs_sum - outputs_sum;

                    datasets.insert(DatasetInsertData {
                        date_data_vec: &date_data_vec,
                        height: block_height,
                        price,
                        coinbase,
                        fees,
                        stxos: &stxos,
                        coinblocks_destroyed,
                        coindays_destroyed,
                    });
                });

            iter_height += blocks_len;

            if (date.day() == 1 || block_count - 1000 < iter_height)
                && iter_height < block_count - NUMBER_OF_UNSAFE_BLOCKS
            {
                export_all(
                    &date,
                    iter_height,
                    &datasets,
                    &date_data_vec,
                    &txid_to_tx_data,
                    &txout_index_to_txout_value,
                    &txid_index_to_block_path,
                )?;
            }

            Ok(())
        })?;

    datasets.export()?;

    Ok(datasets)
}
