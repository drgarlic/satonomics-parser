pub const NUMBER_OF_UNSAFE_BLOCKS: usize = 100;
pub const TARGET_BLOCKS_PER_DAY: usize = 144;
pub const SATOSHIS_PER_BITCOIN: usize = 100_000_000;

pub const BLOCKS_PER_HAVLING_EPOCH: usize = 210_000;

pub const ONE_DAY_IN_BLOCK_TIME: usize = TARGET_BLOCKS_PER_DAY;
pub const ONE_WEEK_IN_BLOCK_TIME: usize = 7 * ONE_DAY_IN_BLOCK_TIME;
pub const TWO_WEEKS_IN_BLOCK_TIME: usize = 2 * ONE_WEEK_IN_BLOCK_TIME;
pub const ONE_MONTH_IN_BLOCK_TIME: usize = 30 * ONE_DAY_IN_BLOCK_TIME;
pub const THREE_MONTHS_IN_BLOCK_TIME: usize = 3 * ONE_MONTH_IN_BLOCK_TIME;
pub const ONE_YEAR_IN_BLOCK_TIME: usize = 12 * ONE_MONTH_IN_BLOCK_TIME;
