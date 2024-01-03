#![allow(dead_code)]

use std::{cell::RefCell, cmp::Ordering};

use bitcoin::Block;
use chrono::Datelike;

use crate::{structs::NUMBER_OF_UNSAFE_BLOCKS, utils::timestamp_to_naive_date};

use super::BlockIter;

pub fn create_group_blocks_by_day_closure() -> impl Fn(&mut BlockIter) -> Option<Vec<Block>> {
    let saved_block: RefCell<Option<Block>> = RefCell::new(None);

    move |iter| {
        let mut blocks: Vec<Block> = vec![];

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

fn create_group_blocks_by_export_event_closure(
    start: usize,
    block_count: usize,
) -> impl Fn(&mut BlockIter) -> Option<Vec<Vec<Block>>> {
    let start = RefCell::new(start);

    let saved_block: RefCell<Option<Block>> = RefCell::new(None);

    move |iter| {
        let mut group: Vec<Vec<Block>> = {
            if let Some(saved_block) = RefCell::take(&saved_block) {
                vec![vec![saved_block]]
            } else {
                vec![vec![]]
            }
        };

        loop {
            let last_blocks = group.last_mut().unwrap();

            if let Some(current_block) = iter.next() {
                *start.borrow_mut() += 1;

                let current_block_date = timestamp_to_naive_date(current_block.header.time);

                if let Some(last_block) = last_blocks.last() {
                    let last_block_date = timestamp_to_naive_date(last_block.header.time);

                    match last_block_date.cmp(&current_block_date) {
                        Ordering::Equal | Ordering::Greater => last_blocks.push(current_block),
                        Ordering::Less => {
                            if current_block_date.day() == 1
                                || block_count - NUMBER_OF_UNSAFE_BLOCKS * 10 < *start.borrow()
                            {
                                saved_block.replace(Some(current_block));

                                break;
                            } else {
                                group.push(vec![current_block])
                            }
                        }
                    }
                } else {
                    last_blocks.push(current_block)
                }
            } else {
                saved_block.replace(None);

                break;
            }
        }

        if group.is_empty() || group.first().unwrap().is_empty() {
            return None;
        }

        Some(group)
    }
}
