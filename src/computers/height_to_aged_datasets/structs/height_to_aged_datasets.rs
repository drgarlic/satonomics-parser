use chrono::Datelike;
use itertools::Itertools;
use rayon::prelude::*;

use super::{AgeRange, BlockDatasPerDay, HeightToAgedDataset};

pub struct HeightToAgedDatasets {
    height_to_1d_dataset: HeightToAgedDataset,
    height_to_7d_dataset: HeightToAgedDataset,
    height_to_1m_dataset: HeightToAgedDataset,
    height_to_3m_dataset: HeightToAgedDataset,
    height_to_6m_dataset: HeightToAgedDataset,
    height_to_1y_dataset: HeightToAgedDataset,
    height_to_2y_dataset: HeightToAgedDataset,
    height_to_3y_dataset: HeightToAgedDataset,
    height_to_5y_dataset: HeightToAgedDataset,
    height_to_7y_dataset: HeightToAgedDataset,
    height_to_10y_dataset: HeightToAgedDataset,
    height_to_all_dataset: HeightToAgedDataset,

    height_to_1d_7d_dataset: HeightToAgedDataset,
    height_to_7d_1m_dataset: HeightToAgedDataset,
    height_to_1m_3m_dataset: HeightToAgedDataset,
    height_to_3m_6m_dataset: HeightToAgedDataset,
    height_to_6m_1y_dataset: HeightToAgedDataset,
    height_to_1y_2y_dataset: HeightToAgedDataset,
    height_to_2y_3y_dataset: HeightToAgedDataset,
    height_to_3y_5y_dataset: HeightToAgedDataset,
    height_to_5y_7y_dataset: HeightToAgedDataset,
    height_to_7y_10y_dataset: HeightToAgedDataset,
    height_to_10y_all_dataset: HeightToAgedDataset,

    height_to_sth_dataset: HeightToAgedDataset,
    height_to_lth_dataset: HeightToAgedDataset,

    height_to_yearly_datasets: Vec<HeightToAgedDataset>,
}

impl HeightToAgedDatasets {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(HeightToAgedDatasets {
            height_to_1d_dataset: HeightToAgedDataset::import("1d", AgeRange::To(1))?,
            height_to_7d_dataset: HeightToAgedDataset::import("7d", AgeRange::To(7))?,
            height_to_1m_dataset: HeightToAgedDataset::import("1m", AgeRange::To(30))?,
            height_to_3m_dataset: HeightToAgedDataset::import("3m", AgeRange::To(3 * 30))?,
            height_to_6m_dataset: HeightToAgedDataset::import("6m", AgeRange::To(6 * 30))?,
            height_to_1y_dataset: HeightToAgedDataset::import("1y", AgeRange::To(365))?,
            height_to_2y_dataset: HeightToAgedDataset::import("2y", AgeRange::To(2 * 365))?,
            height_to_3y_dataset: HeightToAgedDataset::import("3y", AgeRange::To(3 * 365))?,
            height_to_5y_dataset: HeightToAgedDataset::import("5y", AgeRange::To(5 * 365))?,
            height_to_7y_dataset: HeightToAgedDataset::import("7y", AgeRange::To(7 * 365))?,
            height_to_10y_dataset: HeightToAgedDataset::import("10y", AgeRange::To(10 * 365))?,
            height_to_all_dataset: HeightToAgedDataset::import("all", AgeRange::Full)?,

            height_to_1d_7d_dataset: HeightToAgedDataset::import("1d_7d", AgeRange::FromTo(1, 7))?,
            height_to_7d_1m_dataset: HeightToAgedDataset::import("7d_1m", AgeRange::FromTo(7, 30))?,
            height_to_1m_3m_dataset: HeightToAgedDataset::import(
                "1m_3m",
                AgeRange::FromTo(30, 3 * 30),
            )?,
            height_to_3m_6m_dataset: HeightToAgedDataset::import(
                "3m_6m",
                AgeRange::FromTo(3 * 30, 6 * 30),
            )?,
            height_to_6m_1y_dataset: HeightToAgedDataset::import(
                "6m_1y",
                AgeRange::FromTo(6 * 30, 365),
            )?,
            height_to_1y_2y_dataset: HeightToAgedDataset::import(
                "1y_2y",
                AgeRange::FromTo(365, 2 * 365),
            )?,
            height_to_2y_3y_dataset: HeightToAgedDataset::import(
                "2y_3y",
                AgeRange::FromTo(2 * 365, 3 * 365),
            )?,
            height_to_3y_5y_dataset: HeightToAgedDataset::import(
                "3y_5y",
                AgeRange::FromTo(3 * 365, 5 * 365),
            )?,
            height_to_5y_7y_dataset: HeightToAgedDataset::import(
                "5y_7y",
                AgeRange::FromTo(5 * 365, 7 * 365),
            )?,
            height_to_7y_10y_dataset: HeightToAgedDataset::import(
                "7y_10y",
                AgeRange::FromTo(7 * 365, 10 * 365),
            )?,
            height_to_10y_all_dataset: HeightToAgedDataset::import(
                "10y_all",
                AgeRange::From(10 * 365),
            )?,

            height_to_sth_dataset: HeightToAgedDataset::import("sth", AgeRange::To(155))?,
            height_to_lth_dataset: HeightToAgedDataset::import("lth", AgeRange::From(155))?,

            height_to_yearly_datasets: (2009..=(chrono::Utc::now().year() as usize))
                .map(|year| HeightToAgedDataset::import(&year.to_string(), AgeRange::Year(year)))
                .try_collect()?,
        })
    }

    pub fn get_min_last_height(&self) -> Option<usize> {
        self.to_vec()
            .iter()
            .map(|dataset| dataset.get_min_last_height())
            .min()
            .and_then(|opt| opt)
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.to_vec()
            .par_iter()
            .try_for_each(|dataset| dataset.export())?;

        Ok(())
    }

    pub fn insert(
        &self,
        block_datas_per_day: &BlockDatasPerDay,
        current_block_height: usize,
        current_block_price: f32,
    ) {
        self.to_vec().par_iter().for_each(|dataset| {
            dataset.insert(
                block_datas_per_day,
                current_block_height,
                current_block_price,
            )
        });
    }

    fn to_vec(&self) -> Vec<&HeightToAgedDataset> {
        [
            vec![
                &self.height_to_1d_dataset,
                &self.height_to_7d_dataset,
                &self.height_to_1m_dataset,
                &self.height_to_3m_dataset,
                &self.height_to_6m_dataset,
                &self.height_to_1y_dataset,
                &self.height_to_2y_dataset,
                &self.height_to_3y_dataset,
                &self.height_to_5y_dataset,
                &self.height_to_7y_dataset,
                &self.height_to_10y_dataset,
                &self.height_to_all_dataset,
                &self.height_to_1d_7d_dataset,
                &self.height_to_7d_1m_dataset,
                &self.height_to_1m_3m_dataset,
                &self.height_to_3m_6m_dataset,
                &self.height_to_6m_1y_dataset,
                &self.height_to_1y_2y_dataset,
                &self.height_to_2y_3y_dataset,
                &self.height_to_3y_5y_dataset,
                &self.height_to_5y_7y_dataset,
                &self.height_to_7y_10y_dataset,
                &self.height_to_10y_all_dataset,
                &self.height_to_sth_dataset,
                &self.height_to_lth_dataset,
            ],
            self.height_to_yearly_datasets.iter().collect_vec(),
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }
}
