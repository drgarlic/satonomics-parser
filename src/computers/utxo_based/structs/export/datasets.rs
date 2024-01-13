use std::thread;

use chrono::Datelike;
use itertools::Itertools;
use rayon::prelude::*;

use crate::traits::HeightDataset;

use super::{
    dataset_block_metadata::BlockMetadataDataset, AgeRange, AgedDataset, CoinblocksDataset,
    CoindaysDataset, ProcessedData, RewardsDataset,
};

pub struct Datasets {
    height_to_1d_dataset: AgedDataset,
    height_to_7d_dataset: AgedDataset,
    height_to_1m_dataset: AgedDataset,
    height_to_2m_dataset: AgedDataset,
    height_to_3m_dataset: AgedDataset,
    height_to_4m_dataset: AgedDataset,
    height_to_5m_dataset: AgedDataset,
    height_to_6m_dataset: AgedDataset,
    height_to_1y_dataset: AgedDataset,
    height_to_2y_dataset: AgedDataset,
    height_to_3y_dataset: AgedDataset,
    height_to_5y_dataset: AgedDataset,
    height_to_7y_dataset: AgedDataset,
    height_to_10y_dataset: AgedDataset,
    height_to_all_dataset: AgedDataset,

    height_to_1d_7d_dataset: AgedDataset,
    height_to_7d_1m_dataset: AgedDataset,
    height_to_1m_3m_dataset: AgedDataset,
    height_to_3m_6m_dataset: AgedDataset,
    height_to_6m_1y_dataset: AgedDataset,
    height_to_1y_2y_dataset: AgedDataset,
    height_to_2y_3y_dataset: AgedDataset,
    height_to_3y_5y_dataset: AgedDataset,
    height_to_5y_7y_dataset: AgedDataset,
    height_to_7y_10y_dataset: AgedDataset,
    height_to_10y_all_dataset: AgedDataset,

    height_to_sth_dataset: AgedDataset,
    height_to_lth_dataset: AgedDataset,

    height_to_yearly_datasets: Vec<AgedDataset>,

    height_to_rewards: RewardsDataset,

    height_to_coinblocks: CoinblocksDataset,

    height_to_coindays: CoindaysDataset,

    height_to_block_metadata: BlockMetadataDataset,
}

impl Datasets {
    pub fn new() -> color_eyre::Result<Self> {
        let height_to_1d_dataset_handle = thread::spawn(|| AgedDataset::new("1d", AgeRange::To(1)));
        let height_to_7d_dataset_handle = thread::spawn(|| AgedDataset::new("7d", AgeRange::To(7)));
        let height_to_1m_dataset_handle =
            thread::spawn(|| AgedDataset::new("1m", AgeRange::To(30)));
        let height_to_2m_dataset_handle =
            thread::spawn(|| AgedDataset::new("2m", AgeRange::To(2 * 30)));
        let height_to_3m_dataset_handle =
            thread::spawn(|| AgedDataset::new("3m", AgeRange::To(3 * 30)));
        let height_to_4m_dataset_handle =
            thread::spawn(|| AgedDataset::new("4m", AgeRange::To(4 * 30)));
        let height_to_5m_dataset_handle =
            thread::spawn(|| AgedDataset::new("5m", AgeRange::To(5 * 30)));
        let height_to_6m_dataset_handle =
            thread::spawn(|| AgedDataset::new("6m", AgeRange::To(6 * 30)));
        let height_to_1y_dataset_handle =
            thread::spawn(|| AgedDataset::new("1y", AgeRange::To(365)));
        let height_to_2y_dataset_handle =
            thread::spawn(|| AgedDataset::new("2y", AgeRange::To(2 * 365)));
        let height_to_3y_dataset_handle =
            thread::spawn(|| AgedDataset::new("3y", AgeRange::To(3 * 365)));
        let height_to_5y_dataset_handle =
            thread::spawn(|| AgedDataset::new("5y", AgeRange::To(5 * 365)));
        let height_to_7y_dataset_handle =
            thread::spawn(|| AgedDataset::new("7y", AgeRange::To(7 * 365)));
        let height_to_10y_dataset_handle =
            thread::spawn(|| AgedDataset::new("10y", AgeRange::To(10 * 365)));
        let height_to_all_dataset_handle =
            thread::spawn(|| AgedDataset::new("all", AgeRange::Full));

        let height_to_1d_7d_dataset_handle =
            thread::spawn(|| AgedDataset::new("1d_7d", AgeRange::FromTo(1, 7)));
        let height_to_7d_1m_dataset_handle =
            thread::spawn(|| AgedDataset::new("7d_1m", AgeRange::FromTo(7, 30)));
        let height_to_1m_3m_dataset_handle =
            thread::spawn(|| AgedDataset::new("1m_3m", AgeRange::FromTo(30, 3 * 30)));
        let height_to_3m_6m_dataset_handle =
            thread::spawn(|| AgedDataset::new("3m_6m", AgeRange::FromTo(3 * 30, 6 * 30)));
        let height_to_6m_1y_dataset_handle =
            thread::spawn(|| AgedDataset::new("6m_1y", AgeRange::FromTo(6 * 30, 365)));
        let height_to_1y_2y_dataset_handle =
            thread::spawn(|| AgedDataset::new("1y_2y", AgeRange::FromTo(365, 2 * 365)));
        let height_to_2y_3y_dataset_handle =
            thread::spawn(|| AgedDataset::new("2y_3y", AgeRange::FromTo(2 * 365, 3 * 365)));
        let height_to_3y_5y_dataset_handle =
            thread::spawn(|| AgedDataset::new("3y_5y", AgeRange::FromTo(3 * 365, 5 * 365)));
        let height_to_5y_7y_dataset_handle =
            thread::spawn(|| AgedDataset::new("5y_7y", AgeRange::FromTo(5 * 365, 7 * 365)));
        let height_to_7y_10y_dataset_handle =
            thread::spawn(|| AgedDataset::new("7y_10y", AgeRange::FromTo(7 * 365, 10 * 365)));
        let height_to_10y_all_dataset_handle =
            thread::spawn(|| AgedDataset::new("10y_all", AgeRange::From(10 * 365)));

        let height_to_sth_dataset_handle =
            thread::spawn(|| AgedDataset::new("sth", AgeRange::To(155)));
        let height_to_lth_dataset_handle =
            thread::spawn(|| AgedDataset::new("lth", AgeRange::From(155)));

        let height_to_yearly_datasets_handles = (2009..=(chrono::Utc::now().year() as usize))
            .map(|year| {
                thread::spawn(move || AgedDataset::new(&year.to_string(), AgeRange::Year(year)))
            })
            .collect_vec();

        let height_to_rewards_handle = thread::spawn(RewardsDataset::import);

        let height_to_coinblocks_handle = thread::spawn(CoinblocksDataset::import);

        let height_to_coindays_handle = thread::spawn(CoindaysDataset::import);

        let height_to_block_metadata_handle = thread::spawn(BlockMetadataDataset::import);

        Ok(Self {
            height_to_1d_dataset: height_to_1d_dataset_handle.join().unwrap()?,
            height_to_7d_dataset: height_to_7d_dataset_handle.join().unwrap()?,
            height_to_1m_dataset: height_to_1m_dataset_handle.join().unwrap()?,
            height_to_2m_dataset: height_to_2m_dataset_handle.join().unwrap()?,
            height_to_3m_dataset: height_to_3m_dataset_handle.join().unwrap()?,
            height_to_4m_dataset: height_to_4m_dataset_handle.join().unwrap()?,
            height_to_5m_dataset: height_to_5m_dataset_handle.join().unwrap()?,
            height_to_6m_dataset: height_to_6m_dataset_handle.join().unwrap()?,
            height_to_1y_dataset: height_to_1y_dataset_handle.join().unwrap()?,
            height_to_2y_dataset: height_to_2y_dataset_handle.join().unwrap()?,
            height_to_3y_dataset: height_to_3y_dataset_handle.join().unwrap()?,
            height_to_5y_dataset: height_to_5y_dataset_handle.join().unwrap()?,
            height_to_7y_dataset: height_to_7y_dataset_handle.join().unwrap()?,
            height_to_10y_dataset: height_to_10y_dataset_handle.join().unwrap()?,
            height_to_all_dataset: height_to_all_dataset_handle.join().unwrap()?,

            height_to_1d_7d_dataset: height_to_1d_7d_dataset_handle.join().unwrap()?,
            height_to_7d_1m_dataset: height_to_7d_1m_dataset_handle.join().unwrap()?,
            height_to_1m_3m_dataset: height_to_1m_3m_dataset_handle.join().unwrap()?,
            height_to_3m_6m_dataset: height_to_3m_6m_dataset_handle.join().unwrap()?,
            height_to_6m_1y_dataset: height_to_6m_1y_dataset_handle.join().unwrap()?,
            height_to_1y_2y_dataset: height_to_1y_2y_dataset_handle.join().unwrap()?,
            height_to_2y_3y_dataset: height_to_2y_3y_dataset_handle.join().unwrap()?,
            height_to_3y_5y_dataset: height_to_3y_5y_dataset_handle.join().unwrap()?,
            height_to_5y_7y_dataset: height_to_5y_7y_dataset_handle.join().unwrap()?,
            height_to_7y_10y_dataset: height_to_7y_10y_dataset_handle.join().unwrap()?,
            height_to_10y_all_dataset: height_to_10y_all_dataset_handle.join().unwrap()?,

            height_to_sth_dataset: height_to_sth_dataset_handle.join().unwrap()?,
            height_to_lth_dataset: height_to_lth_dataset_handle.join().unwrap()?,

            height_to_yearly_datasets: height_to_yearly_datasets_handles
                .into_par_iter()
                .map(|handle| handle.join().unwrap().unwrap())
                .collect::<Vec<_>>(),

            height_to_rewards: height_to_rewards_handle.join().unwrap()?,

            height_to_coinblocks: height_to_coinblocks_handle.join().unwrap()?,

            height_to_coindays: height_to_coindays_handle.join().unwrap()?,

            height_to_block_metadata: height_to_block_metadata_handle.join().unwrap()?,
        })
    }
    // }

    // impl<'a> HeightDatasets<ProcessedData<'a>> for Datasets {
    pub fn get_min_last_height(&self) -> Option<usize> {
        self.to_vec()
            .iter()
            .map(|dataset| dataset.get_min_last_height())
            .min()
            .and_then(|opt| opt)
    }

    fn export_if_needed(&self, height: Option<usize>) -> color_eyre::Result<()> {
        self.to_vec()
            .par_iter()
            .filter(|dataset| {
                height.is_none()
                    || dataset.get_min_initial_first_unsafe_height().unwrap_or(0) <= height.unwrap()
            })
            .try_for_each(|dataset| dataset.export())
    }

    pub fn export(&self) -> color_eyre::Result<()> {
        self.to_vec()
            .par_iter()
            .try_for_each(|dataset| dataset.export())
    }

    fn insert(&self, processed_data: ProcessedData) {
        let ProcessedData { height, .. } = processed_data;

        self.to_vec()
            .par_iter()
            .filter(|dataset| dataset.get_min_initial_first_unsafe_height().unwrap_or(0) <= height)
            .for_each(|dataset| dataset.insert(&processed_data));
    }

    fn to_vec(&self) -> Vec<&(dyn HeightDataset<ProcessedData> + Send + Sync)> {
        let flat_datasets: Vec<&(dyn HeightDataset<ProcessedData> + Send + Sync)> = vec![
            &self.height_to_1d_dataset,
            &self.height_to_7d_dataset,
            &self.height_to_1m_dataset,
            &self.height_to_2m_dataset,
            &self.height_to_3m_dataset,
            &self.height_to_4m_dataset,
            &self.height_to_5m_dataset,
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
            &self.height_to_rewards,
            &self.height_to_coinblocks,
            &self.height_to_coindays,
            &self.height_to_block_metadata,
        ];

        let yearly_datasets = self
            .height_to_yearly_datasets
            .iter()
            .map(|dataset| dataset as &(dyn HeightDataset<ProcessedData> + Send + Sync))
            .collect_vec();

        [flat_datasets, yearly_datasets]
            .iter()
            .flatten()
            .copied()
            .collect()
    }
}
