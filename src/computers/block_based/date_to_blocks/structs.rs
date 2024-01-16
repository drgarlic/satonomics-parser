use chrono::NaiveDate;

use crate::structs::DateMap;

pub struct DateToBlocks {
    pub date_to_first_block: DateMap<usize>,
    pub date_to_last_block: DateMap<usize>,
    pub date_to_block_count: DateMap<usize>,
}

impl DateToBlocks {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self {
            date_to_first_block: DateMap::import("date_to_first_block.json"),
            date_to_last_block: DateMap::import("date_to_last_block.json"),
            date_to_block_count: DateMap::import("date_to_block_count.json"),
        })
    }

    pub fn get_min_first_unsafe_date(&self) -> Option<NaiveDate> {
        DateMap::get_min_first_unsafe_date(&[
            &self.date_to_first_block,
            &self.date_to_last_block,
            &self.date_to_block_count,
        ])
    }

    pub fn insert(&self, date: &NaiveDate, first_block: usize, blocks_len: usize) {
        self.date_to_first_block.insert(date, first_block);

        self.date_to_last_block
            .insert(date, first_block + (blocks_len - 1).max(0));

        self.date_to_block_count.insert(date, blocks_len);
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.date_to_first_block.export()?;

        self.date_to_last_block.export()?;

        self.date_to_block_count.export()?;

        Ok(())
    }
}
