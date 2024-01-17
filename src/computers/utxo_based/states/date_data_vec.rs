use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};

use crate::computers::utxo_based::{BlockData, DateData};

use super::State;

#[derive(Encode, Decode, Default, Deref, DerefMut, Debug)]
pub struct DateDataVec(Vec<DateData>);

impl DateDataVec {
    pub fn last_mut_block(&mut self) -> &mut BlockData {
        self.last_mut().unwrap().blocks.last_mut().unwrap()
    }
}

impl State for DateDataVec {
    fn name<'a>() -> &'a str {
        "date_data_vec"
    }
}
