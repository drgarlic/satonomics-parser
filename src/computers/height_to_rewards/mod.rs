use bitcoin_explorer::{BitcoinDB, FBlock, FTransaction};

use crate::{structs::TxidToOutputs, utils::convert_sats_to_bitcoins};

pub mod structs;

pub use self::structs::HeightToRewards;

pub fn compute_height_to_rewards(
    db: &BitcoinDB,
    block_count: usize,
) -> color_eyre::Result<HeightToRewards> {
    println!("Computing height_to_rewards...");

    let rewards = HeightToRewards::import()?;

    let start = rewards.get_min_unsafe_height().unwrap_or(0);

    let infinite_txid_to_outputs = TxidToOutputs::new(None);

    let finite_txid_to_outputs = TxidToOutputs::new(Some(25_000_000));

    db.iter_block::<FBlock>(start, block_count)
        .enumerate()
        .try_for_each(|(index, block)| -> color_eyre::Result<()> {
            let height = start + index;
            println!("Height: {height}");

            let coinbase = block
                .txdata
                .first()
                .unwrap()
                .output
                .iter()
                .map(|txout| convert_sats_to_bitcoins(txout.value))
                .sum();

            let fees = block.txdata[1..]
                .iter()
                .map(|tx| {
                    let sum_inputs = tx
                        .input
                        .iter()
                        .map(|txin| {
                            let txid = &txin.previous_output.txid;
                            let vout = txin.previous_output.vout;

                            if let Some(output) = infinite_txid_to_outputs.take(txid, &vout) {
                                return output;
                            }

                            if let Some(output) = finite_txid_to_outputs.take(txid, &vout) {
                                return output;
                            }

                            let tx = db.get_transaction::<FTransaction>(txid).unwrap();

                            finite_txid_to_outputs.insert(&tx);

                            finite_txid_to_outputs.take(txid, &vout).unwrap()
                        })
                        .sum::<f64>();

                    let sum_outputs = tx
                        .output
                        .iter()
                        .map(|txout| convert_sats_to_bitcoins(txout.value))
                        .sum::<f64>();

                    sum_inputs - sum_outputs
                })
                .sum();

            block.txdata.iter().for_each(|tx| {
                infinite_txid_to_outputs.insert(tx);
            });

            rewards.insert(height, coinbase, fees);

            Ok(())
        })?;

    rewards.export()?;

    Ok(rewards)
}
