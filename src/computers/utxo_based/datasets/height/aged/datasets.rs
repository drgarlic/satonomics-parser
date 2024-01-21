use std::thread;

use chrono::Datelike;
use itertools::Itertools;
use rayon::prelude::*;

use crate::computers::utxo_based::{HeightDatasetTrait, HeightDatasetsTrait};

use super::{AgeFilter, AgedDataset};

pub struct AgedDatasets {
    from_start_to_1d: AgedDataset,
    from_start_to_7d: AgedDataset,
    from_start_to_1m: AgedDataset,
    from_start_to_2m: AgedDataset,
    from_start_to_3m: AgedDataset,
    from_start_to_4m: AgedDataset,
    from_start_to_5m: AgedDataset,
    from_start_to_6m: AgedDataset,
    from_start_to_1y: AgedDataset,
    from_start_to_2y: AgedDataset,
    from_start_to_3y: AgedDataset,
    from_start_to_5y: AgedDataset,
    from_start_to_7y: AgedDataset,
    from_start_to_10y: AgedDataset,
    from_start_to_end: AgedDataset,

    from_1d_to_7d: AgedDataset,
    from_7d_to_1m: AgedDataset,
    from_1m_to_3m: AgedDataset,
    from_3m_to_6m: AgedDataset,
    from_6m_to_1y: AgedDataset,
    from_1y_to_2y: AgedDataset,
    from_2y_to_3y: AgedDataset,
    from_3y_to_5y: AgedDataset,
    from_5y_to_7y: AgedDataset,
    from_7y_to_10y: AgedDataset,
    from_10y_to_end: AgedDataset,

    sth: AgedDataset,
    lth: AgedDataset,

    yearly: Vec<AgedDataset>,
}

impl AgedDatasets {
    pub fn import(path: &'static str) -> color_eyre::Result<Self> {
        let from_start_to_1d_handle =
            thread::spawn(|| AgedDataset::import(path, "1d", AgeFilter::To(1)));
        let from_start_to_7d_handle =
            thread::spawn(|| AgedDataset::import(path, "7d", AgeFilter::To(7)));
        let from_start_to_1m_handle =
            thread::spawn(|| AgedDataset::import(path, "1m", AgeFilter::To(30)));
        let from_start_to_2m_handle =
            thread::spawn(|| AgedDataset::import(path, "2m", AgeFilter::To(2 * 30)));
        let from_start_to_3m_handle =
            thread::spawn(|| AgedDataset::import(path, "3m", AgeFilter::To(3 * 30)));
        let from_start_to_4m_handle =
            thread::spawn(|| AgedDataset::import(path, "4m", AgeFilter::To(4 * 30)));
        let from_start_to_5m_handle =
            thread::spawn(|| AgedDataset::import(path, "5m", AgeFilter::To(5 * 30)));
        let from_start_to_6m_handle =
            thread::spawn(|| AgedDataset::import(path, "6m", AgeFilter::To(6 * 30)));
        let from_start_to_1y_handle =
            thread::spawn(|| AgedDataset::import(path, "1y", AgeFilter::To(365)));
        let from_start_to_2y_handle =
            thread::spawn(|| AgedDataset::import(path, "2y", AgeFilter::To(2 * 365)));
        let from_start_to_3y_handle =
            thread::spawn(|| AgedDataset::import(path, "3y", AgeFilter::To(3 * 365)));
        let from_start_to_5y_handle =
            thread::spawn(|| AgedDataset::import(path, "5y", AgeFilter::To(5 * 365)));
        let from_start_to_7y_handle =
            thread::spawn(|| AgedDataset::import(path, "7y", AgeFilter::To(7 * 365)));
        let from_start_to_10y_handle =
            thread::spawn(|| AgedDataset::import(path, "10y", AgeFilter::To(10 * 365)));
        let from_start_to_end_handle =
            thread::spawn(|| AgedDataset::import(path, "all", AgeFilter::Full));

        let from_1d_to_7d_handle =
            thread::spawn(|| AgedDataset::import(path, "1d_7d", AgeFilter::new_from_to(1, 7)));
        let from_7d_to_1m_handle =
            thread::spawn(|| AgedDataset::import(path, "7d_1m", AgeFilter::new_from_to(7, 30)));
        let from_1m_to_3m_handle = thread::spawn(|| {
            AgedDataset::import(path, "1m_3m", AgeFilter::new_from_to(30, 3 * 30))
        });
        let from_3m_to_6m_handle = thread::spawn(|| {
            AgedDataset::import(path, "3m_6m", AgeFilter::new_from_to(3 * 30, 6 * 30))
        });
        let from_6m_to_1y_handle = thread::spawn(|| {
            AgedDataset::import(path, "6m_1y", AgeFilter::new_from_to(6 * 30, 365))
        });
        let from_1y_to_2y_handle = thread::spawn(|| {
            AgedDataset::import(path, "1y_2y", AgeFilter::new_from_to(365, 2 * 365))
        });
        let from_2y_to_3y_handle = thread::spawn(|| {
            AgedDataset::import(path, "2y_3y", AgeFilter::new_from_to(2 * 365, 3 * 365))
        });
        let from_3y_to_5y_handle = thread::spawn(|| {
            AgedDataset::import(path, "3y_5y", AgeFilter::new_from_to(3 * 365, 5 * 365))
        });
        let from_5y_to_7y_handle = thread::spawn(|| {
            AgedDataset::import(path, "5y_7y", AgeFilter::new_from_to(5 * 365, 7 * 365))
        });
        let from_7y_to_10y_handle = thread::spawn(|| {
            AgedDataset::import(path, "7y_10y", AgeFilter::new_from_to(7 * 365, 10 * 365))
        });
        let from_10y_to_end_handle =
            thread::spawn(|| AgedDataset::import(path, "10y_all", AgeFilter::From(10 * 365)));

        let sth_handle = thread::spawn(|| AgedDataset::import(path, "sth", AgeFilter::To(155)));
        let lth_handle = thread::spawn(|| AgedDataset::import(path, "lth", AgeFilter::From(155)));

        let yearly_handles = (2009..=(chrono::Utc::now().year() as usize))
            .map(|year| {
                thread::spawn(move || {
                    AgedDataset::import(path, &year.to_string(), AgeFilter::Year(year))
                })
            })
            .collect_vec();

        Ok(Self {
            from_start_to_1d: from_start_to_1d_handle.join().unwrap()?,
            from_start_to_7d: from_start_to_7d_handle.join().unwrap()?,
            from_start_to_1m: from_start_to_1m_handle.join().unwrap()?,
            from_start_to_2m: from_start_to_2m_handle.join().unwrap()?,
            from_start_to_3m: from_start_to_3m_handle.join().unwrap()?,
            from_start_to_4m: from_start_to_4m_handle.join().unwrap()?,
            from_start_to_5m: from_start_to_5m_handle.join().unwrap()?,
            from_start_to_6m: from_start_to_6m_handle.join().unwrap()?,
            from_start_to_1y: from_start_to_1y_handle.join().unwrap()?,
            from_start_to_2y: from_start_to_2y_handle.join().unwrap()?,
            from_start_to_3y: from_start_to_3y_handle.join().unwrap()?,
            from_start_to_5y: from_start_to_5y_handle.join().unwrap()?,
            from_start_to_7y: from_start_to_7y_handle.join().unwrap()?,
            from_start_to_10y: from_start_to_10y_handle.join().unwrap()?,
            from_start_to_end: from_start_to_end_handle.join().unwrap()?,

            from_1d_to_7d: from_1d_to_7d_handle.join().unwrap()?,
            from_7d_to_1m: from_7d_to_1m_handle.join().unwrap()?,
            from_1m_to_3m: from_1m_to_3m_handle.join().unwrap()?,
            from_3m_to_6m: from_3m_to_6m_handle.join().unwrap()?,
            from_6m_to_1y: from_6m_to_1y_handle.join().unwrap()?,
            from_1y_to_2y: from_1y_to_2y_handle.join().unwrap()?,
            from_2y_to_3y: from_2y_to_3y_handle.join().unwrap()?,
            from_3y_to_5y: from_3y_to_5y_handle.join().unwrap()?,
            from_5y_to_7y: from_5y_to_7y_handle.join().unwrap()?,
            from_7y_to_10y: from_7y_to_10y_handle.join().unwrap()?,
            from_10y_to_end: from_10y_to_end_handle.join().unwrap()?,

            sth: sth_handle.join().unwrap()?,
            lth: lth_handle.join().unwrap()?,

            yearly: yearly_handles
                .into_par_iter()
                .map(|handle| handle.join().unwrap().unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

impl HeightDatasetsTrait for AgedDatasets {
    fn to_vec(&self) -> Vec<&(dyn HeightDatasetTrait + Send + Sync)> {
        let flats: Vec<&(dyn HeightDatasetTrait + Send + Sync)> = vec![
            &self.from_start_to_1d,
            &self.from_start_to_7d,
            &self.from_start_to_1m,
            &self.from_start_to_2m,
            &self.from_start_to_3m,
            &self.from_start_to_4m,
            &self.from_start_to_5m,
            &self.from_start_to_6m,
            &self.from_start_to_1y,
            &self.from_start_to_2y,
            &self.from_start_to_3y,
            &self.from_start_to_5y,
            &self.from_start_to_7y,
            &self.from_start_to_10y,
            &self.from_start_to_end,
            &self.from_1d_to_7d,
            &self.from_7d_to_1m,
            &self.from_1m_to_3m,
            &self.from_3m_to_6m,
            &self.from_6m_to_1y,
            &self.from_1y_to_2y,
            &self.from_2y_to_3y,
            &self.from_3y_to_5y,
            &self.from_5y_to_7y,
            &self.from_7y_to_10y,
            &self.from_10y_to_end,
            &self.sth,
            &self.lth,
        ];

        let yearly = self
            .yearly
            .iter()
            .map(|dataset| dataset as &(dyn HeightDatasetTrait + Send + Sync))
            .collect_vec();

        [flats, yearly].iter().flatten().copied().collect()
    }
}
