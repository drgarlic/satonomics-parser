use std::{
    collections::{BTreeMap, BTreeSet},
    ops::ControlFlow,
    thread,
};

use bitcoin::{Block, Txid};
use chrono::NaiveDate;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use crate::{
    bitcoin::{sats_to_btc, BitcoinDB},
    databases::{
        AddressIndexToEmptyAddressData, Databases, RawAddressToAddressIndex, TxidToTxIndex,
    },
    datasets::{AllDatasets, AnyDatasets, ProcessedBlockData, SortedBlockData},
    states::{SplitRealizedStates, States},
    parse::{
        AddressData, AddressRealizedData, BlockData, BlockPath, Counter, EmptyAddressData,
        PartialTxoutData, RawAddress, TxData, TxoutIndex,
    },
};

pub struct ParseData<'a> {
    pub bitcoin_db: &'a BitcoinDB,
    pub block: Block,
    pub block_index: usize,
    pub coinbase_vec: &'a mut Vec<u64>,
    pub coinblocks_destroyed_vec: &'a mut Vec<f64>,
    pub coindays_destroyed_vec: &'a mut Vec<f64>,
    pub compute_addresses: bool,
    pub databases: &'a mut Databases,
    pub datasets: &'a mut AllDatasets,
    pub date: NaiveDate,
    pub fees_vec: &'a mut Vec<Vec<u64>>,
    pub first_date_height: usize,
    pub height: usize,
    pub is_date_last_block: bool,
    pub sats_sent_vec: &'a mut Vec<u64>,
    pub states: &'a mut States,
    pub subsidy_in_dollars_vec: &'a mut Vec<f32>,
    pub subsidy_vec: &'a mut Vec<u64>,
    pub timestamp: u32,
    pub transaction_count_vec: &'a mut Vec<usize>,
}

pub fn parse_block(
    ParseData {
        bitcoin_db,
        block,
        block_index,
        coinbase_vec,
        coinblocks_destroyed_vec,
        coindays_destroyed_vec,
        compute_addresses,
        databases,
        datasets,
        date,
        fees_vec,
        first_date_height,
        height,
        is_date_last_block,
        sats_sent_vec,
        states,
        subsidy_in_dollars_vec,
        subsidy_vec,
        timestamp,
        transaction_count_vec,
    }: ParseData,
) {
    // println!("block: {height}");

    // If false, expect that the code is flawless
    // TODO: Instead store zero outputs in a database to check
    let enable_check_if_txout_value_is_zero_in_db: bool = true;

    let date_index = states.date_data_vec.len() - 1;

    let block_price = datasets
        .price
        .height_to_close(height, timestamp)
        .unwrap_or_else(|_| panic!("Expect {height} to have a price"));

    let date_price = datasets
        .price
        .date_to_close(date)
        .unwrap_or_else(|_| panic!("Expect {date} to have a price"));

    states
        .date_data_vec
        .last_mut()
        .unwrap()
        .blocks
        .push(BlockData::new(height as u32, block_price));

    let mut block_path_to_spent_value: BTreeMap<BlockPath, u64> = BTreeMap::new();
    let mut address_index_to_address_realized_data: BTreeMap<u32, AddressRealizedData> =
        BTreeMap::new();
    let mut address_index_to_removed_address_data: BTreeMap<u32, AddressData> = BTreeMap::new();

    let mut address_index_at_least_once_removed: BTreeSet<u32> = BTreeSet::default();

    let mut coinbase = 0;
    let mut coinblocks_destroyed = 0.0;
    let mut coindays_destroyed = 0.0;
    let mut sats_sent = 0;
    let mut transaction_count = 0;
    let mut fees = vec![];
    let mut fees_total = 0;

    let (
        (
            TxoutsParsingResults {
                op_returns: _op_returns,
                mut partial_txout_data_vec,
                provably_unspendable: _provably_unspendable,
            },
            mut empty_address_index_to_empty_address_data,
        ),
        mut txin_ordered_tx_indexes,
    ) = thread::scope(|scope| {
        let output_handle = scope.spawn(|| {
            let mut txouts_parsing_results = parse_txouts(
                &block,
                compute_addresses,
                &mut states.counters.unknown_addresses,
                &mut states.counters.empty_addresses,
                &mut databases.raw_address_to_address_index,
            );

            let empty_address_index_to_empty_address_data =
                take_empty_address_index_to_empty_address_data(
                    states,
                    &mut databases.address_index_to_empty_address_data,
                    &txouts_parsing_results.partial_txout_data_vec,
                    compute_addresses,
                );

            // Reverse to get in order via pop later
            txouts_parsing_results.partial_txout_data_vec.reverse();

            (
                txouts_parsing_results,
                empty_address_index_to_empty_address_data,
            )
        });

        let input_handle = scope.spawn(|| {
            let mut txin_ordered_tx_indexes =
                query_txin_ordered_tx_indexes(&block, &mut databases.txid_to_tx_index);

            // Reverse to get in order via pop later
            txin_ordered_tx_indexes.reverse();

            txin_ordered_tx_indexes
        });

        (output_handle.join().unwrap(), input_handle.join().unwrap())
    });

    block.txdata.into_iter().try_for_each(|tx| {
        let txid = tx.txid();
        let txs_counter = &mut databases.txid_to_tx_index.metadata.len;
        let tx_index = txs_counter.inner();
        txs_counter.increment();

        transaction_count += 1;

        // --
        // outputs
        // ---

        let mut spendable_outputs = 0;
        let mut non_zero_amount = 0;

        let is_coinbase = tx.is_coinbase();

        let mut inputs_sum = 0;
        let mut outputs_sum = 0;

        // Before `input` to cover outputs being used in the same block as inputs
        tx.output
            .into_iter()
            .enumerate()
            .filter_map(|(vout, _)| {
                if vout > (u16::MAX as usize) {
                    panic!("vout can indeed be bigger than u16::MAX !");
                }

                partial_txout_data_vec
                    .pop()
                    .unwrap()
                    // None if not worth parsing (empty/op_return/...)
                    .map(|partial_txout_data| (vout, partial_txout_data))
            })
            .for_each(|(vout, partial_txout_data)| {
                let txout_index = TxoutIndex::new(tx_index, vout as u16);

                let PartialTxoutData {
                    raw_address,
                    address_index_opt,
                    sats,
                } = partial_txout_data;

                spendable_outputs += 1;
                non_zero_amount += sats;

                states.txout_index_to_sats.insert(txout_index, sats);

                if compute_addresses {
                    let raw_address = raw_address.unwrap();

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
                                // TODO: Remove after a while
                                if address_data.is_empty() {
                                    panic!("address_data shouldn't be empty");
                                }

                                (address_data, address_index)
                            } else {
                                let empty_address_data = empty_address_index_to_empty_address_data
                                    .remove(&address_index)
                                    .or_else(|| {
                                        address_index_to_removed_address_data
                                            .remove(&address_index);

                                        databases
                                            .address_index_to_empty_address_data
                                            .remove_from_puts(&address_index)
                                    })
                                    .unwrap_or_else(|| {
                                        dbg!(address_index);
                                        panic!("Should've been there");
                                    });

                                let contains_key = states
                                    .address_index_to_address_data
                                    .contains_key(&address_index);

                                if contains_key {
                                    panic!("Shouldn't be anything there");
                                }

                                databases
                                    .address_index_to_empty_address_data
                                    .metadata
                                    .len
                                    .decrement();

                                let address_data = states
                                    .address_index_to_address_data
                                    .entry(address_index)
                                    // Will always insert, it's to avoid insert + get
                                    .or_insert(AddressData::from_empty(&empty_address_data));

                                (address_data, address_index)
                            }
                        } else {
                            let addresses_counters =
                                &mut databases.raw_address_to_address_index.metadata.len;

                            let address_index = addresses_counters.inner();

                            addresses_counters.increment();

                            let address_type = raw_address.to_type();

                            if let Some(previous) = databases
                                .raw_address_to_address_index
                                .insert(raw_address, address_index)
                            {
                                dbg!(previous);
                                panic!("address #{address_index} shouldn't be present during put");
                            }

                            let address_data = states
                                .address_index_to_address_data
                                .entry(address_index)
                                // Will always insert, it's to avoid insert + get
                                .or_insert(AddressData::new(address_type));

                            (address_data, address_index)
                        }
                    };

                    // MUST be before received !
                    let address_realized_data = address_index_to_address_realized_data
                        .entry(address_index)
                        .or_insert_with(|| AddressRealizedData::default(address_data));

                    address_data.receive(sats, block_price);

                    address_realized_data.received += sats;

                    states
                        .txout_index_to_address_index
                        .insert(txout_index, address_index);
                }
            });

        if is_coinbase {
            coinbase = non_zero_amount;
            coinbase_vec.push(coinbase);
        } else {
            outputs_sum += non_zero_amount;
        }

        let last_block = states.date_data_vec.last_mut_block();

        last_block.amount += non_zero_amount;

        if spendable_outputs != 0 {
            last_block.spendable_outputs += spendable_outputs as u32;

            databases.txid_to_tx_index.insert(&txid, tx_index);

            states.tx_index_to_tx_data.insert(
                tx_index,
                TxData::new(
                    BlockPath::new(date_index as u16, block_index as u16),
                    spendable_outputs,
                ),
            );
        }

        // ---
        // inputs
        // ---

        if !is_coinbase {
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
                        if !enable_check_if_txout_value_is_zero_in_db
                            || check_if_txout_value_is_zero_in_db(
                                bitcoin_db,
                                &input_txid,
                                input_vout,
                            )
                        {
                            return ControlFlow::Continue::<()>(());
                        }

                        dbg!((input_txid, txid, tx_index, input_vout));
                        panic!("Txid to be in txid_to_tx_data");
                    }

                    let input_tx_index = input_tx_index.unwrap();

                    let input_txout_index = TxoutIndex::new(input_tx_index, input_vout as u16);

                    let input_sats = states.txout_index_to_sats.remove(&input_txout_index);

                    if input_sats.is_none() {
                        if !enable_check_if_txout_value_is_zero_in_db
                            || check_if_txout_value_is_zero_in_db(
                                bitcoin_db,
                                &input_txid,
                                input_vout,
                            )
                        {
                            return ControlFlow::Continue::<()>(());
                        }

                        dbg!((input_txid, tx_index, input_tx_index, input_vout, input_sats,));
                        panic!("Txout index to be in txout_index_to_txout_value");
                    }

                    let input_sats = input_sats.unwrap();
                    let input_btc = sats_to_btc(input_sats);

                    let input_tx_data =
                        states.tx_index_to_tx_data.get_mut(&input_tx_index).unwrap();

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

                    input_block_data.spendable_outputs -= 1;

                    input_block_data.amount -= input_sats;

                    inputs_sum += input_sats;

                    *block_path_to_spent_value
                        .entry(input_block_path)
                        .or_default() += input_sats;

                    coinblocks_destroyed +=
                        (height - input_block_data.height as usize) as f64 * input_btc;

                    coindays_destroyed +=
                        date.signed_duration_since(*input_date_data.date).num_days() as f64
                            * input_btc;

                    input_tx_data.spendable_outputs -= 1;

                    if compute_addresses {
                        let input_address_index = states
                            .txout_index_to_address_index
                            .remove(&input_txout_index)
                            .unwrap();

                        let input_address_is_empty = {
                            let input_address_data = states
                                .address_index_to_address_data
                                .get_mut(&input_address_index)
                                .unwrap_or_else(|| {
                                    dbg!(input_address_index);
                                    panic!();
                                });

                            let input_address_realized_data =
                                address_index_to_address_realized_data
                                    .entry(input_address_index)
                                    .or_insert_with(|| {
                                        AddressRealizedData::default(input_address_data)
                                    });

                            input_address_realized_data.sent += input_sats;

                            let address_realized_profit_or_loss =
                                input_address_data.spend(input_sats, input_block_data.price);

                            if address_realized_profit_or_loss >= 0.0 {
                                input_address_realized_data.profit +=
                                    address_realized_profit_or_loss;
                            } else {
                                input_address_realized_data.loss +=
                                    address_realized_profit_or_loss.abs();
                            }

                            input_address_data.is_empty()
                        };

                        if input_address_is_empty {
                            let input_address_data = states
                                .address_index_to_address_data
                                .remove(&input_address_index)
                                .unwrap();

                            address_index_at_least_once_removed.insert(input_address_index);

                            databases.address_index_to_empty_address_data.insert(
                                input_address_index,
                                EmptyAddressData::from_non_empty(&input_address_data),
                            );

                            address_index_to_removed_address_data
                                .insert(input_address_index, input_address_data);

                            databases
                                .address_index_to_empty_address_data
                                .metadata
                                .len
                                .increment();
                        }
                    }

                    if input_tx_data.is_empty() {
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
            })?;
        }

        sats_sent += inputs_sum;

        let fee = inputs_sum - outputs_sum;
        fees_total += fee;
        fees.push(fee);

        ControlFlow::Continue(())
    });

    let subsidy = coinbase - fees_total;
    let subsidy_in_dollars = (sats_to_btc(subsidy) * block_price as f64) as f32;

    coinblocks_destroyed_vec.push(coinblocks_destroyed);
    coindays_destroyed_vec.push(coindays_destroyed);
    sats_sent_vec.push(sats_sent);
    transaction_count_vec.push(transaction_count);
    fees_vec.push(fees.clone());
    subsidy_vec.push(subsidy);
    subsidy_in_dollars_vec.push(subsidy_in_dollars);

    let mut split_realized_states = None;
    let mut split_price_paid_states = None;
    let mut split_unrealized_states_height = None;
    let mut split_unrealized_states_date = None;
    let mut sorted_block_data_vec = None;

    thread::scope(|scope| {
        if datasets.utxo.needs_sorted_block_data_vec(date, height) {
            scope.spawn(|| {
                let date_data_vec = &states.date_data_vec;
                let len = date_data_vec.len() as u16;

                let mut vec = date_data_vec
                    .par_iter()
                    .flat_map(|date_data| {
                        date_data
                            .blocks
                            .par_iter()
                            .map(move |block_data| SortedBlockData {
                                reversed_date_index: date_data.reverse_index(len),
                                year: date_data.year,
                                block_data,
                            })
                    })
                    .collect::<Vec<_>>();

                vec.par_sort_unstable_by(|a, b| {
                    Ord::cmp(
                        &OrderedFloat(a.block_data.price),
                        &OrderedFloat(b.block_data.price),
                    )
                });

                sorted_block_data_vec.replace(vec);
            });
        }

        if compute_addresses {
            scope.spawn(|| {
                split_realized_states.replace(SplitRealizedStates::default());
                let processed_addresses_split_states = &mut states.split_address;

                address_index_to_address_realized_data.iter().for_each(
                    |(address_index, address_realized_data)| {
                        let current_address_data = states
                            .address_index_to_address_data
                            .get(address_index)
                            .unwrap_or_else(|| {
                                address_index_to_removed_address_data
                                    .get(address_index)
                                    .unwrap()
                            });

                        processed_addresses_split_states
                            .iterate(address_realized_data, current_address_data);

                        split_realized_states
                            .as_mut()
                            .unwrap()
                            .iterate_realized(address_realized_data);
                    },
                );

                split_price_paid_states
                    .replace(processed_addresses_split_states.compute_price_paid_states());

                split_unrealized_states_height.replace(
                    processed_addresses_split_states.compute_unrealized_states(block_price),
                );

                if is_date_last_block {
                    split_unrealized_states_date.replace(
                        processed_addresses_split_states.compute_unrealized_states(date_price),
                    );
                }
            });
        }
    });

    datasets.insert_block_data(ProcessedBlockData {
        address_index_to_address_realized_data: &address_index_to_address_realized_data,
        address_index_to_removed_address_data: &address_index_to_removed_address_data,
        block_path_to_spent_value: &block_path_to_spent_value,
        block_price,
        coinbase,
        coinbase_vec,
        coinblocks_destroyed_vec,
        coindays_destroyed_vec,
        databases,
        date,
        date_price,
        fees: &fees,
        fees_vec,
        first_date_height,
        height,
        is_date_last_block,
        sats_sent,
        sats_sent_vec,
        sorted_block_data_vec,
        split_price_paid_states: &split_price_paid_states,
        split_realized_states: &mut split_realized_states,
        split_unrealized_states_date: &split_unrealized_states_date,
        split_unrealized_states_height: &split_unrealized_states_height,
        states,
        subsidy,
        subsidy_in_dollars,
        subsidy_in_dollars_vec,
        subsidy_vec,
        timestamp,
        transaction_count,
        transaction_count_vec,
    });
}

fn check_if_txout_value_is_zero_in_db(bitcoin_db: &BitcoinDB, txid: &Txid, vout: u32) -> bool {
    bitcoin_db
        .get_transaction(txid)
        .unwrap()
        .output
        .get(vout as usize)
        .unwrap()
        .to_owned()
        .value
        .to_sat()
        == 0
}

pub struct TxoutsParsingResults {
    partial_txout_data_vec: Vec<Option<PartialTxoutData>>,
    provably_unspendable: u64,
    op_returns: usize,
}

fn parse_txouts(
    block: &Block,
    compute_addresses: bool,
    unknown_addresses: &mut Counter,
    empty_addresses: &mut Counter,
    raw_address_to_address_index: &mut RawAddressToAddressIndex,
) -> TxoutsParsingResults {
    let mut provably_unspendable = 0;
    let mut op_returns = 0;

    let mut partial_txout_data_vec = block
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

            let raw_address_opt = {
                if compute_addresses {
                    let raw_address = RawAddress::from(txout, unknown_addresses, empty_addresses);

                    raw_address_to_address_index.open_db(&raw_address);

                    Some(raw_address)
                } else {
                    None
                }
            };

            Some(PartialTxoutData::new(raw_address_opt, value, None))
        })
        .collect_vec();

    if compute_addresses {
        partial_txout_data_vec.par_iter_mut().for_each(|opt| {
            if let Some(partial_tx_out_data) = opt {
                partial_tx_out_data.address_index_opt = raw_address_to_address_index
                    .unsafe_get(partial_tx_out_data.raw_address.as_ref().unwrap())
                    .cloned();
            }
        });
    }

    TxoutsParsingResults {
        partial_txout_data_vec,
        provably_unspendable,
        op_returns,
    }
}

fn take_empty_address_index_to_empty_address_data(
    states: &mut States,
    address_index_to_empty_address_data: &mut AddressIndexToEmptyAddressData,
    partial_txout_data_vec: &[Option<PartialTxoutData>],
    compute_addresses: bool,
) -> BTreeMap<u32, EmptyAddressData> {
    if !compute_addresses {
        return BTreeMap::default();
    }

    let address_index_to_address_data = &mut states.address_index_to_address_data;

    let mut empty_address_index_to_empty_address_data = partial_txout_data_vec
        .iter()
        .flatten()
        .flat_map(|partial_txout_data| partial_txout_data.address_index_opt)
        .flat_map(|address_index| {
            if address_index_to_address_data.contains_key(&address_index) {
                None
            } else {
                address_index_to_empty_address_data.open_db(&address_index);

                Some((address_index, EmptyAddressData::default()))
            }
        })
        .collect::<BTreeMap<_, _>>();

    empty_address_index_to_empty_address_data
        .par_iter_mut()
        .for_each(|(address_index, empty_address_data)| {
            empty_address_data.copy(
                address_index_to_empty_address_data
                    .unsafe_get(address_index)
                    .unwrap(),
            );
        });

    // Parallel unsafe_get + Linear remove = Parallel-ish take
    empty_address_index_to_empty_address_data
        .keys()
        .for_each(|address_index| {
            address_index_to_empty_address_data.remove(address_index);
        });

    empty_address_index_to_empty_address_data
}

fn query_txin_ordered_tx_indexes(
    block: &Block,
    txid_to_tx_index: &mut TxidToTxIndex,
) -> Vec<Option<u32>> {
    block
        .txdata
        .iter()
        // Skip coinbase transaction
        .skip(1)
        .flat_map(|tx| &tx.input)
        .for_each(|txin| {
            txid_to_tx_index.open_db(&txin.previous_output.txid);
        });

    block
        .txdata
        .par_iter()
        .skip(1)
        .flat_map(|tx| &tx.input)
        .map(|txin| {
            txid_to_tx_index
                .unsafe_get(&txin.previous_output.txid)
                .cloned()
        })
        .collect::<Vec<_>>()
}
