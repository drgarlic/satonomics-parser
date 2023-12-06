#![allow(dead_code)]

use std::cell::RefCell;

use bitcoin_explorer::{BlockIter, FBlock};

use crate::utils::timestamp_to_naive_date;

pub fn create_group_blocks_by_day_closure() -> impl Fn(&mut BlockIter<FBlock>) -> Option<Vec<FBlock>>
{
    let saved_block: RefCell<Option<FBlock>> = RefCell::new(None);

    move |iter| {
        let mut blocks: Vec<FBlock> = vec![];

        let mut saved_block_date = {
            if let Some(saved_block) = RefCell::take(&saved_block) {
                let saved_block_date = timestamp_to_naive_date(saved_block.header.time);

                blocks.push(saved_block);

                Some(saved_block_date)
            } else {
                None
            }
        };

        loop {
            if let Some(block) = iter.next() {
                let block_date = timestamp_to_naive_date(block.header.time);

                let saved_block_date = saved_block_date.get_or_insert(block_date).to_owned();

                if saved_block_date < block_date {
                    saved_block.replace(Some(block));
                    break;
                } else {
                    blocks.push(block);
                }
            } else {
                saved_block.replace(None);
                break;
            }
        }

        if blocks.is_empty() {
            return None;
        }

        Some(blocks)
    }
}
