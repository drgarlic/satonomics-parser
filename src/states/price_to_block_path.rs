use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};
use ordered_float::OrderedFloat;

use crate::parse::BlockPath;

use super::DateDataVec;

#[derive(Deref, DerefMut, Default)]
pub struct PriceToBlockPathVec(BTreeMap<OrderedFloat<f32>, Vec<BlockPath>>);

impl PriceToBlockPathVec {
    pub fn build(date_data_vec: &DateDataVec) -> Self {
        Self(
            date_data_vec
                .iter()
                .enumerate()
                .flat_map(|(date_index, date_data)| {
                    date_data
                        .blocks
                        .iter()
                        .enumerate()
                        .map(move |(block_index, block_data)| {
                            (
                                OrderedFloat(block_data.price),
                                BlockPath {
                                    date_index: date_index as u16,
                                    block_index: block_index as u16,
                                },
                            )
                        })
                })
                .fold(BTreeMap::new(), |mut tree, (price, block_path)| {
                    tree.entry(price).or_default().push(block_path);
                    tree
                }),
        )
    }
}
