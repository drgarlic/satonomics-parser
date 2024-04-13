use savefile_derive::Savefile;

#[derive(Savefile, Debug)]
pub struct BlockData {
    pub height: u32,
    pub price: f32,
    pub timestamp: u32,
    pub amount: u64,
    pub spendable_outputs: u32,
}

impl BlockData {
    pub fn new(height: u32, price: f32, timestamp: u32) -> Self {
        Self {
            height,
            price,
            timestamp,
            amount: 0,
            spendable_outputs: 0,
        }
    }
}
