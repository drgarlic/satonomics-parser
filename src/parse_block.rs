use std::{collections::BTreeMap, ops::ControlFlow};

use bitcoin::{Block, TxOut, Txid};
use chrono::{Datelike, NaiveDate};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use crate::{
    bitcoin::{sats_to_btc, BitcoinDB},
    databases::Databases,
    datasets::{AllDatasets, AnyDatasets, ProcessedBlockData},
    states::States,
    structs::{
        AddressData, AddressRealizedData, BlockData, BlockPath, EmptyAddressData, PartialTxoutData,
        RawAddress, TxData, TxoutIndex,
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
    pub height: usize,
    pub is_date_last_block: bool,
    pub states: &'a mut States,
    pub timestamp: u32,
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
        height,
        is_date_last_block,
        states,
        timestamp,
    }: ParseData,
) {
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

    fees_vec.push(vec![]);

    let mut block_path_to_spent_value: BTreeMap<BlockPath, u64> = BTreeMap::new();
    let mut address_index_to_address_realized_data: BTreeMap<u32, AddressRealizedData> =
        BTreeMap::new();
    let mut address_index_to_removed_address_data: BTreeMap<u32, AddressData> = BTreeMap::new();

    let mut coinblocks_destroyed = 0.0;
    let mut coindays_destroyed = 0.0;

    let TxoutsParsingResults {
        mut partial_txout_data_vec,
        op_returns: _op_returns,
        provably_unspendable: _provably_unspendable,
    } = parse_txouts(&block, states, databases, compute_addresses);

    let mut empty_address_index_to_empty_address_data =
        take_empty_address_index_to_empty_address_data(
            states,
            databases,
            &partial_txout_data_vec,
            compute_addresses,
        );

    // Reverse to get in order via pop later
    partial_txout_data_vec.reverse();

    let mut txin_ordered_tx_indexes = query_txin_ordered_tx_indexes(&block, databases);

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

        let mut spendable_outputs = 0;
        let mut non_zero_amount = 0;

        let is_coinbase = tx.is_coinbase();

        let mut inputs_sum = 0;
        let mut outputs_sum = 0;

        // Before `input` as some inputs can be used as later outputs
        tx.output
            .into_iter()
            .enumerate()
            .map(|(vout, _)| vout)
            .try_for_each(|vout| -> ControlFlow<()> {
                if vout > (u16::MAX as usize) {
                    panic!("vout can indeed be bigger than u16::MAX !");
                }

                let txout_index = TxoutIndex::new(tx_index, vout as u16);

                let partial_txout_data = partial_txout_data_vec.pop().unwrap();

                if partial_txout_data.is_none() {
                    return ControlFlow::Continue(());
                }

                let PartialTxoutData {
                    raw_address,
                    address_index_opt,
                    sats,
                } = partial_txout_data.unwrap();

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
                                (address_data, address_index)
                            } else {
                                let empty_address_data = empty_address_index_to_empty_address_data
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

                                let previous = states.address_index_to_address_data.insert(
                                    address_index,
                                    AddressData::from_empty(&empty_address_data),
                                );

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

                    address_data.receive(sats, block_price);

                    address_index_to_address_realized_data
                        .entry(address_index)
                        .or_default()
                        .received += sats;

                    states
                        .txout_index_to_address_index
                        .insert(txout_index, address_index);
                }

                ControlFlow::Continue(())
            });

        if spendable_outputs != 0 {
            databases.txid_to_tx_index.insert(&txid, tx_index);

            states.tx_index_to_tx_data.insert(
                tx_index,
                TxData::new(
                    BlockPath::new(date_index as u16, block_index as u16),
                    spendable_outputs,
                ),
            );

            let last_block = states.date_data_vec.last_mut_block();

            last_block.amount += non_zero_amount;
            last_block.spendable_outputs += spendable_outputs as u32;

            if is_coinbase {
                coinbase_vec.push(non_zero_amount);
            } else {
                outputs_sum += non_zero_amount;
            }
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

                    let input_sats = states.txout_index_to_sats.remove(&input_txout_index);

                    if input_sats.is_none() {
                        let txout_from_db = get_txout_from_db(bitcoin_db, &input_txid, input_vout);

                        if txout_from_db.value.to_sat() == 0 {
                            return ControlFlow::Continue::<()>(());
                        } else {
                            dbg!((
                                input_txid,
                                tx_index,
                                input_tx_index,
                                input_vout,
                                input_sats,
                                txout_from_db
                            ));
                            panic!("Txout index to be in txout_index_to_txout_value");
                        }
                    }

                    let input_sats = input_sats.unwrap();

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

                    coinblocks_destroyed += (height - input_block_data.height as usize) as f64
                        * sats_to_btc(input_block_data.amount);

                    coindays_destroyed +=
                        date.signed_duration_since(*input_date_data.date).num_days() as f64
                            * sats_to_btc(input_block_data.amount);

                    input_tx_data.spendable_outputs -= 1;

                    if compute_addresses {
                        let input_address_index = states
                            .txout_index_to_address_index
                            .remove(&input_txout_index)
                            .unwrap();

                        let input_address_data = states
                            .address_index_to_address_data
                            .get_mut(&input_address_index)
                            .unwrap_or_else(|| {
                                dbg!(input_address_index);
                                panic!();
                            });

                        let address_realized_profit_or_loss =
                            input_address_data.spend(input_sats, input_block_data.price);

                        if input_address_data.is_empty() {
                            let address_data = states
                                .address_index_to_address_data
                                .remove(&input_address_index)
                                .unwrap();

                            databases.address_index_to_empty_address_data.insert(
                                input_address_index,
                                EmptyAddressData::from_non_empty(&address_data),
                            );

                            address_index_to_removed_address_data
                                .insert(input_address_index, address_data);
                        }

                        let address_realized_data = &mut address_index_to_address_realized_data
                            .entry(input_address_index)
                            .or_default();

                        address_realized_data.sent += input_sats;

                        if address_realized_profit_or_loss >= 0.0 {
                            address_realized_data.profit += address_realized_profit_or_loss;
                        } else {
                            address_realized_data.loss += address_realized_profit_or_loss.abs();
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

        let fees = inputs_sum - outputs_sum;

        fees_vec.last_mut().unwrap().push(fees);

        ControlFlow::Continue(())
    });

    address_index_to_address_realized_data
        .par_iter_mut()
        .for_each(|(address_index, address_realized_data)| {
            let address_data = states
                .address_index_to_address_data
                .get(address_index)
                .unwrap_or_else(|| {
                    address_index_to_removed_address_data
                        .get(address_index)
                        .unwrap()
                });

            address_realized_data.address_data_opt.replace(address_data);

            address_realized_data.previous_amount_opt.replace(
                address_data.amount + address_realized_data.sent - address_realized_data.received,
            );
        });

    let sorted_address_data = {
        if compute_addresses && datasets.address.needs_sorted_address_data(date, height) {
            let mut vec = states.address_index_to_address_data.values().collect_vec();

            vec.par_sort_unstable_by(|a, b| {
                Ord::cmp(
                    &OrderedFloat(a.mean_price_paid),
                    &OrderedFloat(b.mean_price_paid),
                )
            });

            Some(vec)
        } else {
            None
        }
    };

    let sorted_block_data_vec = {
        if datasets.utxo.needs_sorted_block_data_vec(date, height) {
            let date_data_vec = &states.date_data_vec;
            let len = date_data_vec.len();

            let mut sorted_block_data_vec = date_data_vec
                .iter()
                .enumerate()
                .map(|(index, date_data)| (len - index - 1, date_data))
                .flat_map(|(reversed_index, date_data)| {
                    date_data
                        .blocks
                        .iter()
                        .map(move |block_data| (reversed_index, date_data.date.year(), block_data))
                })
                .collect_vec();

            sorted_block_data_vec.par_sort_unstable_by(|a, b| {
                Ord::cmp(&OrderedFloat(a.2.price), &OrderedFloat(b.2.price))
            });

            Some(sorted_block_data_vec)
        } else {
            None
        }
    };

    coinblocks_destroyed_vec.push(coinblocks_destroyed);
    coindays_destroyed_vec.push(coindays_destroyed);

    datasets.insert_block_data(ProcessedBlockData {
        address_index_to_address_realized_data: &address_index_to_address_realized_data,
        address_index_to_removed_address_data: &address_index_to_removed_address_data,
        block_path_to_spent_value: &block_path_to_spent_value,
        block_price,
        coinbase_vec,
        coinblocks_destroyed_vec,
        coindays_destroyed_vec,
        date,
        date_price,
        fees_vec,
        height,
        is_date_last_block,
        sorted_address_data,
        sorted_block_data_vec,
        states,
        timestamp,
    });
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
    block: &Block,
    states: &mut States,
    databases: &mut Databases,
    compute_addresses: bool,
) -> TxoutsParsingResults {
    let raw_address_to_address_index = &mut databases.raw_address_to_address_index;

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
                    let raw_address = RawAddress::from(txout, states);
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
    databases: &mut Databases,
    partial_txout_data_vec: &[Option<PartialTxoutData>],
    compute_addresses: bool,
) -> BTreeMap<u32, EmptyAddressData> {
    if !compute_addresses {
        return BTreeMap::default();
    }

    let address_index_to_empty_address_data = &mut databases.address_index_to_empty_address_data;

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

fn query_txin_ordered_tx_indexes(block: &Block, databases: &mut Databases) -> Vec<Option<u32>> {
    let txid_to_tx_index = &mut databases.txid_to_tx_index;

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
