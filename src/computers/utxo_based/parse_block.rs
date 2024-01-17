use std::{collections::BTreeMap, ops::ControlFlow};

use bitcoin::{Block, TxOut, Txid};
use chrono::NaiveDate;
use itertools::Itertools;
use nohash_hasher::IntSet;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use crate::{
    bitcoin::{sats_to_btc, BitcoinDB},
    computers::utxo_based::structs::{
        AddressData, BlockData, BlockPath, TxData, TxoutData, TxoutIndex,
    },
};

use super::{
    structs::EmptyAddressData, Databases, Datasets, PartialTxoutData, ProcessedData, RawAddress,
    Snapshots, States,
};

pub struct ParseData<'a> {
    pub bitcoin_db: &'a BitcoinDB,
    pub block: Block,
    pub block_count: usize,
    pub block_index: usize,
    pub databases: &'a mut Databases,
    pub datasets: &'a mut Datasets,
    pub date: NaiveDate,
    pub height: usize,
    pub height_to_price: &'a [f32],
    pub timestamp: u32,
    pub snapshots: &'a Snapshots,
    pub states: &'a mut States,
}

pub fn parse_block(
    ParseData {
        bitcoin_db,
        block,
        block_count,
        block_index,
        databases,
        datasets,
        date,
        height,
        height_to_price,
        timestamp,
        snapshots,
        states,
    }: ParseData,
) {
    let date_index = states.date_data_vec.len() - 1;

    let price = height_to_price
        .get(height)
        .unwrap_or_else(|| panic!("Expect {height} to have a price"))
        .to_owned();

    states
        .date_data_vec
        .last_mut()
        .unwrap()
        .blocks
        .push(BlockData::new(height as u32, price));

    let mut coinbase = 0;
    let mut inputs_sum = 0;
    let mut outputs_sum = 0;

    let mut block_path_to_spent_value: BTreeMap<BlockPath, u64> = BTreeMap::new();
    let mut address_index_to_spent_value: BTreeMap<u32, BTreeMap<OrderedFloat<f32>, u64>> =
        BTreeMap::new();

    let mut coinblocks_destroyed = 0.0;
    let mut coindays_destroyed = 0.0;

    let TxoutsParsingResults {
        mut partial_txout_data_vec,
        op_returns,
        provably_unspendable,
    } = parse_txouts(height, block_count, &block, states, databases, snapshots);

    let mut cached_address_index_to_empty_address_data = query_address_index_to_empty_address_data(
        height,
        block_count,
        states,
        databases,
        snapshots,
        &partial_txout_data_vec,
    );

    // Reverse to get in order via pop later
    partial_txout_data_vec.reverse();

    let mut txin_ordered_tx_indexes =
        query_txin_ordered_tx_indexes(height, block_count, &block, databases, snapshots);

    // Reverse to get in order via pop later
    txin_ordered_tx_indexes.reverse();

    block.txdata.into_iter().try_for_each(|tx| {
        let txid = tx.txid();
        let txs_counter = &mut states.counters.txs;
        let tx_index = txs_counter.inner();
        txs_counter.increment();

        // --
        // outputs
        // ---

        let mut non_zero_outputs_len = 0;
        let mut non_zero_amount = 0;

        let is_coinbase = tx.is_coinbase();

        // Before `input` as some inputs can be used as later outputs
        tx.output
            .into_iter()
            .enumerate()
            .map(|(vout, _)| vout)
            .try_for_each(|vout| -> ControlFlow<()> {
                if vout > (u16::MAX as usize) {
                    panic!("vout can indeed be bigger than u16::MAX !");
                }

                non_zero_outputs_len += 1;

                let txout_index = TxoutIndex::new(tx_index, vout as u16);

                let tmp_address_data = partial_txout_data_vec.pop().unwrap();

                if tmp_address_data.is_none() {
                    return ControlFlow::Continue(());
                }

                let PartialTxoutData {
                    raw_address,
                    address_index_opt,
                    value,
                } = tmp_address_data.unwrap();

                non_zero_amount += value;

                let (address_data, address_index) = {
                    if let Some(address_index) = address_index_opt.or_else(|| {
                        databases
                            .raw_address_to_address_index
                            .unsafe_get_from_puts(&raw_address)
                            .cloned()
                    }) {
                        if let Some(address_data) =
                            states.address_index_to_address_data.get_mut(&address_index)
                        {
                            (address_data, address_index)
                        } else {
                            let empty_address_data = cached_address_index_to_empty_address_data
                                .remove(&address_index)
                                .or_else(|| {
                                    databases
                                        .address_index_to_empty_address_data
                                        .remove_from_puts(&address_index)
                                })
                                .unwrap_or_else(|| {
                                    dbg!(address_index);
                                    panic!("Should've been there");
                                });

                            let previous = states
                                .address_index_to_address_data
                                .insert(address_index, AddressData::from_empty(empty_address_data));

                            if previous.is_some() {
                                dbg!(address_index);
                                panic!("Shouldn't be anything there");
                            }

                            let address_data = states
                                .address_index_to_address_data
                                .get_mut(&address_index)
                                .unwrap();

                            (address_data, address_index)
                        }
                    } else {
                        let addresses_counters = &mut states.counters.addresses;

                        let address_index = addresses_counters.inner();

                        addresses_counters.increment();

                        let address_type = raw_address.to_type();

                        let previous = databases
                            .raw_address_to_address_index
                            .insert(raw_address, address_index);

                        if previous.is_some() {
                            dbg!(previous);
                            panic!("address #{address_index} shouldn't be present during put");
                        }

                        states
                            .address_index_to_address_data
                            .insert(address_index, AddressData::new(address_type));

                        let address_data = states
                            .address_index_to_address_data
                            .get_mut(&address_index)
                            .unwrap();

                        (address_data, address_index)
                    }
                };

                address_data.receive(value, price);

                states
                    .txout_index_to_txout_data
                    .insert(txout_index, TxoutData::new(value, address_index));

                ControlFlow::Continue(())
            });

        if non_zero_outputs_len != 0 {
            databases.txid_to_tx_index.insert(&txid, tx_index);

            states.tx_index_to_tx_data.insert(
                tx_index,
                TxData::new(
                    BlockPath::new(date_index as u16, block_index as u16),
                    non_zero_outputs_len,
                ),
            );

            let last_block = states.date_data_vec.last_mut_block();

            last_block.amount += non_zero_amount;
            last_block.outputs_len += non_zero_outputs_len as u32;

            if is_coinbase {
                coinbase = non_zero_amount;
            } else {
                outputs_sum += non_zero_amount;
            }
        }

        // ---
        // inputs
        // ---

        if is_coinbase {
            return ControlFlow::Continue::<()>(());
        }

        tx.input.into_iter().try_for_each(|txin| {
            let outpoint = txin.previous_output;
            let input_txid = outpoint.txid;
            let input_vout = outpoint.vout;

            let input_tx_index = {
                let input_tx_index = txin_ordered_tx_indexes.pop().unwrap().or_else(|| {
                    databases
                        .txid_to_tx_index
                        .unsafe_get_from_puts(&input_txid)
                        .cloned()
                });

                if input_tx_index.is_none() {
                    let txout_from_db = get_txout_from_db(bitcoin_db, &input_txid, input_vout);

                    if txout_from_db.value.to_sat() == 0 {
                        return ControlFlow::Continue::<()>(());
                    } else {
                        dbg!((input_txid, txid, tx_index, input_vout, txout_from_db));
                        panic!("Txid to be in txid_to_tx_data");
                    }
                }

                let input_tx_index = input_tx_index.unwrap();

                let input_txout_index = TxoutIndex::new(input_tx_index, input_vout as u16);

                let input_txout_data = states.txout_index_to_txout_data.remove(&input_txout_index);

                if input_txout_data.is_none() {
                    let txout_from_db = get_txout_from_db(bitcoin_db, &input_txid, input_vout);

                    if txout_from_db.value.to_sat() == 0 {
                        return ControlFlow::Continue::<()>(());
                    } else {
                        dbg!((
                            input_txid,
                            tx_index,
                            input_tx_index,
                            input_vout,
                            input_txout_data,
                            txout_from_db
                        ));
                        panic!("Txout index to be in txout_index_to_txout_value");
                    }
                }

                let input_txout_data = input_txout_data.unwrap();
                let input_txout_value = input_txout_data.value;

                let input_tx_data = states.tx_index_to_tx_data.get_mut(&input_tx_index).unwrap();

                let input_block_path = input_tx_data.block_path;

                let BlockPath {
                    date_index: input_date_index,
                    block_index: input_block_index,
                } = input_block_path;

                let input_date_data = states
                    .date_data_vec
                    .get_mut(input_date_index as usize)
                    .unwrap_or_else(|| {
                        dbg!(height, &input_txid, input_block_path, input_date_index);
                        panic!()
                    });

                let input_block_data = input_date_data
                    .blocks
                    .get_mut(input_block_index as usize)
                    .unwrap_or_else(|| {
                        dbg!(
                            height,
                            &input_txid,
                            input_block_path,
                            input_date_index,
                            input_block_index,
                        );
                        panic!()
                    });

                input_block_data.outputs_len -= 1;

                input_block_data.amount -= input_txout_value;

                inputs_sum += input_txout_value;

                *block_path_to_spent_value
                    .entry(input_block_path)
                    .or_default() += input_txout_value;

                let input_address_index = input_txout_data.address_index;

                let move_address_to_empty = {
                    let address_data = states
                        .address_index_to_address_data
                        .get_mut(&input_address_index)
                        .unwrap_or_else(|| {
                            dbg!(input_address_index);
                            panic!();
                        });

                    address_data.spend(input_txout_value, input_block_data.price);

                    address_data.is_empty()
                };

                if move_address_to_empty {
                    let address_data = states
                        .address_index_to_address_data
                        .remove(&input_address_index)
                        .unwrap();

                    databases.address_index_to_empty_address_data.insert(
                        input_address_index,
                        EmptyAddressData::from_non_empty(address_data),
                    );
                }

                *address_index_to_spent_value
                    .entry(input_address_index)
                    .or_default()
                    .entry(OrderedFloat(input_block_data.price))
                    .or_default() += input_txout_value;

                coinblocks_destroyed += (height - input_block_data.height as usize) as f64
                    * sats_to_btc(input_block_data.amount);

                coindays_destroyed += date.signed_duration_since(*input_date_data.date).num_days()
                    as f64
                    * sats_to_btc(input_block_data.amount);

                input_tx_data.outputs_len -= 1;

                if input_tx_data.outputs_len == 0 {
                    Some(input_tx_index)
                } else {
                    None
                }
            };

            if let Some(input_tx_index) = input_tx_index {
                states.tx_index_to_tx_data.remove(&input_tx_index);
                databases.txid_to_tx_index.remove(&input_txid);
            }

            ControlFlow::Continue(())
        });

        ControlFlow::Continue(())
    });

    let fees = inputs_sum - outputs_sum;

    // datasets.insert(ProcessedData {
    //     address_index_to_address_data: &address_index_to_address_data,
    //     address_index_to_spent_value: &address_index_to_spent_value,
    //     block_path_to_spent_value: &block_path_to_spent_value,
    //     coinbase,
    //     coinblocks_destroyed,
    //     coindays_destroyed,
    //     date,
    //     date_data_vec: &date_data_vec,
    //     fees,
    //     height,
    //     price,
    //     timestamp,
    // });
}

fn get_txout_from_db(bitcoin_db: &BitcoinDB, txid: &Txid, vout: u32) -> TxOut {
    bitcoin_db
        .get_transaction(txid)
        .unwrap()
        .output
        .get(vout as usize)
        .unwrap()
        .to_owned()
}

pub struct TxoutsParsingResults {
    partial_txout_data_vec: Vec<Option<PartialTxoutData>>,
    provably_unspendable: u64,
    op_returns: usize,
}

fn parse_txouts(
    #[allow(unused_variables)] height: usize,
    #[allow(unused_variables)] block_count: usize,
    block: &Block,
    states: &mut States,
    databases: &mut Databases,
    #[allow(unused_variables)] snapshots: &Snapshots,
) -> TxoutsParsingResults {
    let raw_address_to_address_index = &mut databases.raw_address_to_address_index;

    let mut provably_unspendable = 0;
    let mut op_returns = 0;

    let incomplete_txout_data_vec = block
        .txdata
        .iter()
        .flat_map(|tx| &tx.output)
        .map(|txout| {
            let script = &txout.script_pubkey;
            let value = txout.value.to_sat();

            // 0 sats outputs are possible and allowed !
            // https://mempool.space/tx/2f2442f68e38b980a6c4cec21e71851b0d8a5847d85208331a27321a9967bbd6
            // https://bitcoin.stackexchange.com/questions/104937/transaction-outputs-with-value-0
            if value == 0 {
                return None;
            }

            // https://mempool.space/tx/8a68c461a2473653fe0add786f0ca6ebb99b257286166dfb00707be24716af3a#flow=&vout=0
            if script.is_provably_unspendable() {
                provably_unspendable += value;
                return None;
            }

            // https://mempool.space/tx/fd0d23d88059dd3b285ede0c88a1246b880e9d8cbac8aa0077a37d70091769d1#flow=&vout=2
            if script.is_op_return() {
                // TODO: Count fee paid to write said OP_RETURN, beware of coinbase transactions
                // For coinbase transactions, count miners
                op_returns += 1;
                return None;
            }

            let raw_address = RawAddress::from(txout, states);

            raw_address_to_address_index.open_db(&raw_address);

            Some((raw_address, value))
        })
        .collect_vec();

    // let txout_ordered_address_indexes_snapshot =
    //     snapshots.txout_ordered_address_indexes.import(height);

    let partial_txout_data_vec = incomplete_txout_data_vec
        .into_par_iter()
        // .enumerate()
        .map(|opt| {
            opt.map(|(raw_address, value)| {
                let address_index_opt = {
                    // if let Ok(txout_ordered_address_indexes_snapshot) =
                    //     &txout_ordered_address_indexes_snapshot
                    // {
                    //     txout_ordered_address_indexes_snapshot
                    //         .get(index)
                    //         .unwrap_or_else(|| {
                    //             dbg!(
                    //                 &txout_ordered_address_indexes_snapshot,
                    //                 index,
                    //                 &txout_ordered_address_indexes_snapshot.len()
                    //             );
                    //             panic!();
                    //         })
                    //         .to_owned()
                    // } else {
                    raw_address_to_address_index
                        .unsafe_get(&raw_address)
                        .cloned()
                    // }
                };

                PartialTxoutData::new(raw_address, value, address_index_opt)
            })
        })
        .collect::<Vec<_>>();

    // if txout_ordered_address_indexes_snapshot.is_err() && check_if_height_safe(height, block_count)
    // {
    //     let txout_ordered_address_indexes = partial_txout_data_vec
    //         .iter()
    //         .map(|partial_txout_data_opt| {
    //             partial_txout_data_opt
    //                 .as_ref()
    //                 .and_then(|partial_txout_data| partial_txout_data.address_index_opt)
    //         })
    //         .collect_vec();

    //     let _ = snapshots
    //         .txout_ordered_address_indexes
    //         .export(height, &txout_ordered_address_indexes);
    // }

    TxoutsParsingResults {
        partial_txout_data_vec,
        provably_unspendable,
        op_returns,
    }
}

fn query_address_index_to_empty_address_data(
    #[allow(unused_variables)] height: usize,
    #[allow(unused_variables)] block_count: usize,
    states: &mut States,
    databases: &mut Databases,
    #[allow(unused_variables)] snapshots: &Snapshots,
    partial_txout_data_vec: &[Option<PartialTxoutData>],
) -> BTreeMap<u32, EmptyAddressData> {
    let address_index_to_empty_address_data = &mut databases.address_index_to_empty_address_data;

    let address_index_to_address_data = &mut states.address_index_to_address_data;

    let empty_address_indexes = partial_txout_data_vec
        .iter()
        .flatten()
        .flat_map(|partial_txout_data| partial_txout_data.address_index_opt)
        .flat_map(|address_index| {
            if address_index_to_address_data.contains_key(&address_index) {
                None
            } else {
                address_index_to_empty_address_data.open_db(&address_index);
                Some(address_index)
            }
        })
        .collect::<IntSet<_>>();

    // let empty_address_index_to_empty_address_data_snapshot = snapshots
    //     .empty_address_index_to_empty_address_data
    //     .import(height);

    let cached_address_index_to_empty_address_data = empty_address_indexes
        .into_par_iter()
        .map(|address_index| {
            let empty_address = {
                // if let Ok(empty_address_index_to_empty_address_data_snapshot) =
                //     &empty_address_index_to_empty_address_data_snapshot
                // {
                //     empty_address_index_to_empty_address_data_snapshot
                //         .get(&address_index)
                //         .unwrap()
                // } else {
                address_index_to_empty_address_data
                    .unsafe_get(&address_index)
                    .unwrap()
                // }
            };

            (address_index.to_owned(), empty_address.to_owned())
        })
        .collect::<BTreeMap<_, _>>();

    // if empty_address_index_to_empty_address_data_snapshot.is_err()
    //     && check_if_height_safe(height, block_count)
    // {
    //     let _ = snapshots
    //         .empty_address_index_to_empty_address_data
    //         .export(height, &cached_address_index_to_empty_address_data);
    // }

    // Parallel unsafe_get + Linear remove = Parallel-ish take
    cached_address_index_to_empty_address_data
        .keys()
        .for_each(|address_index| {
            address_index_to_empty_address_data.remove(address_index);
        });

    cached_address_index_to_empty_address_data
}

fn query_txin_ordered_tx_indexes(
    #[allow(unused_variables)] height: usize,
    #[allow(unused_variables)] block_count: usize,
    block: &Block,
    databases: &mut Databases,
    #[allow(unused_variables)] snapshots: &Snapshots,
) -> Vec<Option<u32>> {
    let txid_to_tx_index = &mut databases.txid_to_tx_index;

    // let txins =
    block
        .txdata
        .iter()
        // Skip coinbase transaction
        .skip(1)
        .flat_map(|tx| &tx.input)
        .for_each(|txin| {
            txid_to_tx_index.open_db(&txin.previous_output.txid);
            // txin
        });
    // .collect_vec();

    // let txin_ordered_tx_indexes_snapshot = snapshots.txin_ordered_tx_indexes.import(height);

    let txin_ordered_tx_indexes = block
        .txdata
        .iter()
        // Skip coinbase transaction
        .skip(1)
        .flat_map(|tx| &tx.input)
        // txins
        // .par_iter()
        // .enumerate()
        .map(|txin| {
            // if let Ok(txin_ordered_tx_indexes_snapshot) = &txin_ordered_tx_indexes_snapshot {
            //     txin_ordered_tx_indexes_snapshot
            //         .get(index)
            //         .unwrap()
            //         .to_owned()
            // } else {
            txid_to_tx_index
                .unsafe_get(&txin.previous_output.txid)
                .cloned()
            // }
        })
        .collect::<Vec<_>>();

    // if txin_ordered_tx_indexes_snapshot.is_err() && check_if_height_safe(height, block_count) {
    //     let _ = snapshots
    //         .txin_ordered_tx_indexes
    //         .export(height, &txin_ordered_tx_indexes);
    // }

    txin_ordered_tx_indexes
}
