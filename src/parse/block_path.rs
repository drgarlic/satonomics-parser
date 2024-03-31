use savefile_derive::Savefile;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, Savefile)]
pub struct BlockPath {
    pub date_index: u16,
    pub block_index: u16,
}

impl BlockPath {
    pub fn new(date_index: u16, block_index: u16) -> Self {
        Self {
            date_index,
            block_index,
        }
    }
}
