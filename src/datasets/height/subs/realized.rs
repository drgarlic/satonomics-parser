use std::fs;

use crate::structs::{AnyHeightMap, HeightMap};

pub struct RealizedDataset {
    /// NOTE: Fees not taken into account
    profit: HeightMap<f32>,
    /// NOTE: Fees not taken into account
    loss: HeightMap<f32>,
}

impl RealizedDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/realized");

        fs::create_dir_all(&folder_path)?;

        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            profit: HeightMap::new_on_disk_bin(&f("profit")),
            loss: HeightMap::new_on_disk_bin(&f("loss")),
        })
    }

    pub fn insert(&self, height: usize, realized_loss: f32, realized_profit: f32) {
        self.profit.insert(height, realized_profit);
        self.loss.insert(height, realized_loss);
    }

    pub fn to_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.profit, &self.loss]
    }
}
