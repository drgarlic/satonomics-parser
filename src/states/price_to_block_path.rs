use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};
use ordered_float::OrderedFloat;
use rayon::prelude::*;

use crate::parse::BlockPath;

use super::DateDataVec;

#[derive(Deref, DerefMut, Default)]
pub struct PriceToBlockPath(BTreeMap<OrderedFloat<f32>, BlockPath>);

impl PriceToBlockPath {
    pub fn build(date_data_vec: &DateDataVec) -> Self {
        Self(
            date_data_vec
                .par_iter()
                .enumerate()
                .flat_map(|(date_index, date_data)| {
                    date_data
                        .blocks
                        .par_iter()
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
                .collect(),
        )
    }
}
