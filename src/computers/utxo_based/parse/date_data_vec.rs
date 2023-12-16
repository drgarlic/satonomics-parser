use std::ops::{Deref, DerefMut};

use bincode::{Decode, Encode};

use crate::utils::Snapshot;

use super::{BlockData, DateData};

#[derive(Encode, Decode)]
pub struct DateDataVec(Vec<DateData>);

impl DateDataVec {
    pub fn default() -> Self {
        Self(vec![])
    }

    pub fn last_mut_block(&mut self) -> &mut BlockData {
        self.last_mut().unwrap().blocks.last_mut().unwrap()
    }
}

impl Snapshot<DateDataVec> for DateDataVec {
    fn name<'a>() -> &'a str {
        "height_to_aged__date_data_vec"
    }
}

impl Deref for DateDataVec {
    type Target = Vec<DateData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateDataVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
