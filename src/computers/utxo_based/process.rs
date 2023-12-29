use std::{collections::BTreeMap, ops::ControlFlow};

use bitcoin_explorer::{BitcoinDB, FBlock, FTransaction};
use chrono::NaiveDate;
use heed::RwTxn;

use crate::{
    computers::utxo_based::structs::{
        AddressData, BlockData, BlockPath, TxData, TxoutData, TxoutIndex,
    },
    structs::WTxid,
    utils::{addresses_to_bytes, convert_sats_to_bitcoins},
};

use super::structs::{
    // AddressIndexToAddress,
    AddressIndexToAddressData,
    AddressIndexToEmptyAddressData,
    AddressToAddressIndex,
    DateDataVec,
    EmptyAddressData,
    // EmptyAddressToAddressIndex,
    TxIndexToTxData,
    TxidToTxIndex,
    TxoutIndexToTxoutData,
};

pub struct ProcessData<'a, 'env> {
    pub address_counter: &'a mut u32,
    // pub address_index_to_address: &'a mut AddressIndexToAddress,
    pub address_index_to_address_data: &'a mut AddressIndexToAddressData,
    pub address_index_to_empty_address_data: &'a mut AddressIndexToEmptyAddressData,
    pub address_to_address_index: &'a mut AddressToAddressIndex,
    pub bitcoin_db: &'a BitcoinDB,
    pub block: FBlock,
    pub block_height: usize,
    pub block_index: usize,
    pub date: NaiveDate,
    pub date_data_vec: &'a mut DateDataVec,
    // pub empty_address_to_address_index: &'a mut EmptyAddressToAddressIndex,
    pub height_to_price: &'a [f32],
    pub tx_counter: &'a mut u32,
    pub tx_index_to_tx_data: &'a mut TxIndexToTxData,
    pub txid_to_tx_index: &'a mut TxidToTxIndex,
    pub txout_index_to_txout_data: &'a mut TxoutIndexToTxoutData,
    pub writer: &'a mut RwTxn<'env>,
}

pub fn process_block(
    ProcessData {
        address_counter,
        // address_index_to_address,
        address_index_to_address_data,
        address_to_address_index,
        address_index_to_empty_address_data,
        bitcoin_db,
        block,
        block_height,
        block_index,
        date,
        date_data_vec,
        // empty_address_to_address_index,
        height_to_price,
        tx_counter,
        tx_index_to_tx_data,
        txid_to_tx_index,
        txout_index_to_txout_data,
        writer,
    }: ProcessData,
) {
    let date_index = date_data_vec.len() - 1;

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

    let mut block_path_to_spent_value = BTreeMap::new();
    let mut address_index_to_spent_value = BTreeMap::new();

    let mut coinblocks_destroyed = 0.0;
    let mut coindays_destroyed = 0.0;

    block.txdata.into_iter().for_each(|tx| {
        let wtxid = WTxid::wrap(tx.txid);

        let tx_index = *tx_counter;

        *tx_counter += 1;

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
            .filter(|(_, txout)| txout.value > 0)
            .try_for_each(|(vout, txout)| {
                if vout > (u16::MAX as usize) {
                    panic!("vout can indeed be bigger than u16::MAX !");
                }

                let addresses = txout.addresses;

                let address_bytes = match addresses.len() {
                    // TODO: Count number of utxos sent to no man's land
                    0 => return ControlFlow::Continue::<()>(()),
                    _ => addresses_to_bytes(&addresses),
                };

                let address_bytes = address_bytes.as_slice();

                non_zero_outputs_len += 1;

                let txout_index = TxoutIndex::new(tx_index, vout as u16);

                let txout_value = convert_sats_to_bitcoins(txout.value);

                non_zero_amount += txout_value;

                let (address_data, address_index) = {
                    if let Some(address_index) = address_to_address_index.get(writer, address_bytes)
                    {
                        if let Some(address_data) =
                            address_index_to_address_data.get_mut(&address_index)
                        {
                            (address_data, address_index)
                        } else {
                            let empty_address_data = address_index_to_empty_address_data
                                .take(writer, &address_index)
                                .unwrap();

                            let previous = address_index_to_address_data
                                .insert(address_index, AddressData::from_empty(empty_address_data));

                            if previous.is_some() {
                                dbg!(address_index);
                                panic!("Shouldn't be anything there");
                            }

                            let address_data = address_index_to_address_data
                                .get_mut(&address_index)
                                .unwrap();

                            (address_data, address_index)
                        }

                    // } else if let Some(address_index) = empty_address_to_address_index
                    //     .get(writer, address_bytes)
                    //     .unwrap()
                    // {
                    //     empty_address_to_address_index
                    //         .delete(writer, address_bytes)
                    //         .unwrap();

                    //     let empty_address_data = address_index_to_empty_address_data
                    //         .get(writer, &address_index)
                    //         .unwrap()
                    //         .unwrap();

                    //     address_index_to_empty_address_data
                    //         .delete(writer, &address_index)
                    //         .unwrap();

                    //     address_to_address_index
                    //         .put(writer, address_bytes, &address_index)
                    //         .unwrap();

                    //     address_index_to_address
                    //         .put(writer, &address_index, address_bytes)
                    //         .unwrap();

                    //     address_index_to_address_data
                    //         .insert(address_index, AddressData::from_empty(empty_address_data));

                    //     let address_data = address_index_to_address_data
                    //         .get_mut(&address_index)
                    //         .unwrap();

                    //     (address_data, address_index)
                    } else {
                        let address_index = *address_counter;
                        *address_counter += 1;

                        address_to_address_index.put(address_bytes, address_index);

                        address_index_to_address_data.insert(address_index, AddressData::default());

                        let address_data = address_index_to_address_data
                            .get_mut(&address_index)
                            .unwrap();

                        (address_data, address_index)
                    }
                };

                address_data.receive(txout_value, price);

                txout_index_to_txout_data
                    .insert(txout_index, TxoutData::new(txout_value, address_index));

                ControlFlow::Continue(())
            });

        if non_zero_outputs_len != 0 {
            txid_to_tx_index.insert(wtxid, tx_index);

            tx_index_to_tx_data.insert(
                tx_index,
                TxData::new(
                    BlockPath::new(date_index as u16, block_index as u16),
                    non_zero_outputs_len,
                ),
            );

            let last_block = date_data_vec.last_mut_block();

            last_block.amount += non_zero_amount;
            last_block.outputs_len += non_zero_outputs_len as u32;

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
            let input_wtxid = WTxid::wrap(outpoint.txid);
            let input_vout = outpoint.vout;

            let tx_index_to_remove = {
                let input_tx_index = txid_to_tx_index.get(&input_wtxid).cloned();

                let get_txout_from_db = || {
                    bitcoin_db
                        .get_transaction::<FTransaction>(&input_wtxid)
                        .unwrap()
                        .output
                        .get(input_vout as usize)
                        .unwrap()
                        .to_owned()
                };

                if input_tx_index.is_none() {
                    let txout_from_db = get_txout_from_db();

                    if txout_from_db.value == 0 || txout_from_db.addresses.len() == 0 {
                        return ControlFlow::Continue::<()>(());
                    } else {
                        dbg!((wtxid, input_wtxid, tx_index, input_vout, txout_from_db));
                        panic!("Txid to be in txid_to_tx_data");
                    }
                }

                let input_tx_index = input_tx_index.unwrap();

                let input_txout_index = TxoutIndex::new(input_tx_index, input_vout as u16);

                let input_txout_data = txout_index_to_txout_data.remove(&input_txout_index);

                if input_txout_data.is_none() {
                    let txout_from_db = get_txout_from_db();

                    if txout_from_db.value == 0 || txout_from_db.addresses.len() == 0 {
                        return ControlFlow::Continue::<()>(());
                    } else {
                        dbg!((
                            wtxid,
                            input_wtxid,
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
                let input_address_index = input_txout_data.address_index;

                let input_tx_data = tx_index_to_tx_data.get_mut(&input_tx_index).unwrap();

                let input_block_path = input_tx_data.block_path;

                let BlockPath {
                    date_index: input_date_index,
                    block_index: input_block_index,
                } = input_block_path;

                let input_date_data = date_data_vec
                    .get_mut(input_date_index as usize)
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
                    .get_mut(input_block_index as usize)
                    .unwrap_or_else(|| {
                        dbg!(
                            block_height,
                            &input_wtxid,
                            input_block_path,
                            input_date_index,
                            input_block_index,
                        );
                        panic!()
                    });

                input_block_data.outputs_len -= 1;

                input_block_data.amount -= input_txout_value;

                inputs_sum += input_txout_value;

                block_path_to_spent_value.insert(
                    input_block_path,
                    block_path_to_spent_value
                        .get(&input_block_path)
                        .unwrap_or(&0.0)
                        + input_txout_value,
                );

                let move_address_to_empty = {
                    let address_data = address_index_to_address_data
                        .get_mut(&input_address_index)
                        .unwrap_or_else(|| {
                            // let address = address_index_to_address
                            //     .get(writer, &input_address_index)
                            //     .unwrap();

                            // dbg!(input_address_index, address);
                            dbg!(input_address_index);
                            panic!();
                        });

                    address_data.spend(input_txout_value, input_block_data.price);

                    address_data.is_empty()
                };

                if move_address_to_empty {
                    // let address = address_index_to_address
                    //     .get(writer, &input_address_index)
                    //     .unwrap()
                    //     .unwrap()
                    //     .to_owned();

                    // address_index_to_address
                    //     .delete(writer, &input_address_index)
                    //     .unwrap();

                    // address_to_address_index.delete(writer, &address).unwrap();

                    // empty_address_to_address_index
                    //     .put(writer, &address, &input_address_index)
                    //     .unwrap();

                    let address_data = address_index_to_address_data
                        .remove(&input_address_index)
                        .unwrap();

                    address_index_to_empty_address_data.put(
                        input_address_index,
                        EmptyAddressData::from_non_empty(address_data),
                    );
                }

                address_index_to_spent_value.insert(
                    input_address_index,
                    address_index_to_spent_value
                        .get(&input_address_index)
                        .unwrap_or(&0.0)
                        + input_txout_value,
                );

                coinblocks_destroyed += (block_height - input_block_data.height as usize) as f64
                    * input_block_data.amount;

                coindays_destroyed += date.signed_duration_since(*input_date_data.date).num_days()
                    as f64
                    * input_block_data.amount;

                input_tx_data.outputs_len -= 1;

                if input_tx_data.outputs_len == 0 {
                    Some(input_tx_index)
                } else {
                    None
                }
            };

            if let Some(tx_index_to_remove) = tx_index_to_remove {
                tx_index_to_tx_data.remove(&tx_index_to_remove);
                txid_to_tx_index.remove(&input_wtxid);
            }

            ControlFlow::Continue(())
        });
    });

    let _fees = inputs_sum - outputs_sum;

    // datasets.insert(DatasetInsertData {
    //     date_data_vec: &date_data_vec,
    //     address_index_to_address_data: &address_index_to_address_data,
    //     height: block_height,
    //     price,
    //     coinbase,
    //     fees,
    //     coinblocks_destroyed,
    //     coindays_destroyed,
    //     block_path_to_spent_value: &block_path_to_spent_value,
    //     address_index_to_spent_value: &address_index_to_spent_value,
    // });
}
