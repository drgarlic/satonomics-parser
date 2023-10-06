use bitcoin_explorer::{BitcoinDB, FBlock, Txid};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, path::Path};

use crate::utils::{import_json_array, push_to_saved_vec};

const BLOCK_NUMBER_PER_BLOCK_DAY: usize = 144;
const BLOCK_NUMBER_PER_DIFFICULTY_ADJUSTMENT: usize = BLOCK_NUMBER_PER_BLOCK_DAY * 14;
const BLOCK_NUMBER_PER_CHUNK: usize = BLOCK_NUMBER_PER_DIFFICULTY_ADJUSTMENT * 2;

#[derive(Serialize, Deserialize)]
struct ChunkIO {
    index: usize,
    outputs_sum: usize,
    inputs_ages: HashMap<usize, usize>,
}

pub fn process(
    db: &BitcoinDB,
    block_count: usize,
    outputs_folder_rawpath: &str,
) -> color_eyre::Result<()> {
    let save_path = Path::new(outputs_folder_rawpath).join("chunks.json");

    let txid_to_height: RefCell<HashMap<Txid, usize>> = RefCell::new(HashMap::new());

    let already_processed_chunks_number = {
        let arr: Vec<ChunkIO> = import_json_array(&save_path, true)?;
        arr.len()
    };

    db.iter_block::<FBlock>(
        BLOCK_NUMBER_PER_CHUNK * already_processed_chunks_number,
        block_count,
    )
    .chunks(BLOCK_NUMBER_PER_CHUNK)
    .into_iter()
    .enumerate()
    .try_for_each(|(index, chunk)| -> color_eyre::Result<()> {
        let index = index + already_processed_chunks_number;

        let chunk_height = index * BLOCK_NUMBER_PER_CHUNK;

        println!("Chunk {index} - Height: {chunk_height}...");

        let mut outputs_sum = 0;

        let inputs_ages = chunk
            .into_iter()
            .enumerate()
            .flat_map(|(index, block)| {
                let height = chunk_height + index;

                println!("Block {height}");

                block
                    .txdata
                    .iter()
                    .flat_map(|tx| {
                        outputs_sum += tx.output.len();

                        {
                            txid_to_height.borrow_mut().insert(tx.txid, height);
                        };

                        tx.input
                            .iter()
                            .map(|txin| txin.previous_output.txid)
                            .filter(|txid| !txid.eq(&Txid::default()))
                            .map(|txid| {
                                let tx_height = {
                                    let height_opt = {
                                        txid_to_height
                                            .borrow()
                                            .get(&txid)
                                            .to_owned()
                                            .map(|h| h.to_owned())
                                    };

                                    if let Some(tx_height) = height_opt {
                                        tx_height
                                    } else {
                                        let tx_height =
                                            db.get_height_of_transaction(&txid).unwrap();

                                        {
                                            txid_to_height.borrow_mut().insert(txid, tx_height);
                                        };

                                        tx_height
                                    }
                                };

                                (height - tx_height) / BLOCK_NUMBER_PER_BLOCK_DAY
                            })
                    })
                    .collect::<Vec<_>>()
            })
            .fold(HashMap::new(), |mut map, age_in_block_days| {
                let previous_value = map.get(&age_in_block_days).unwrap_or(&usize::MIN);

                map.insert(age_in_block_days, previous_value + 1);

                map
            });

        let chunk_io = ChunkIO {
            index,
            outputs_sum,
            inputs_ages,
        };

        push_to_saved_vec(&save_path, chunk_io)?;

        Ok(())
    })?;

    Ok(())
}
