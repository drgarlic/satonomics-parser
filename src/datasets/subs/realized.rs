use crate::{
    datasets::ProcessedBlockData,
    structs::{AnyDateMap, AnyHeightMap, BiMap},
};

/// NOTE: Fees not taken into account
pub struct RealizedSubDataset {
    profit: BiMap<f32>,
    loss: BiMap<f32>,
}

impl RealizedSubDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let folder_path = format!("{parent_path}/realized");
        let f = |s: &str| format!("{folder_path}/{s}");

        Ok(Self {
            profit: BiMap::new_on_disk_bin(&f("profit")),
            loss: BiMap::new_on_disk_bin(&f("loss")),
        })
    }

    pub fn insert(
        &self,
        &ProcessedBlockData {
            date,
            height,
            is_date_last_block,
            ..
        }: &ProcessedBlockData,
        realized_loss: f32,
        realized_profit: f32,
    ) {
        self.profit.height.insert(height, realized_profit);
        self.loss.height.insert(height, realized_loss);

        if is_date_last_block {
            self.profit.date.insert(date, realized_profit);
            self.loss.date.insert(date, realized_loss);
        }
    }

    pub fn to_any_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.profit.height, &self.loss.height]
    }

    pub fn to_any_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.profit.date, &self.loss.date]
    }
}
