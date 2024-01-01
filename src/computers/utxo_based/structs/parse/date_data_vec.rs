use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::traits::Snapshot;

use super::{BlockData, DateData};

#[derive(Encode, Decode, Default, Deref, DerefMut)]
pub struct DateDataVec(Vec<DateData>);

impl DateDataVec {
    pub fn last_mut_block(&mut self) -> &mut BlockData {
        self.last_mut().unwrap().blocks.last_mut().unwrap()
    }
}

impl Snapshot for DateDataVec {
    fn name<'a>() -> &'a str {
        "date_data_vec"
    }
}
