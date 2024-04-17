mod dataset;

use dataset::*;

use std::thread;

use itertools::Itertools;

use crate::{
    datasets::AnyDatasets,
    states::{SplitByUTXOCohort, UTXOCohortId},
};

use super::{AnyDataset, MinInitialState, ProcessedBlockData};

pub struct UTXODatasets {
    min_initial_state: MinInitialState,

    cohorts: SplitByUTXOCohort<UTXODataset>,
}

impl UTXODatasets {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        thread::scope(|scope| {
            let up_to_1d_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo1d));
            let up_to_1w_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo1w));
            let up_to_1m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo1m));
            let up_to_2m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo2m));
            let up_to_3m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo3m));
            let up_to_4m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo4m));
            let up_to_5m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo5m));
            let up_to_6m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo6m));
            let up_to_1y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo1y));
            let up_to_2y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo2y));
            let up_to_3y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo3y));
            let up_to_5y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo5y));
            let up_to_7y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo7y));
            let up_to_10y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::UpTo10y));

            let from_1d_to_1w_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From1dTo1w));
            let from_1w_to_1m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From1wTo1m));
            let from_1m_to_3m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From1mTo3m));
            let from_3m_to_6m_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From3mTo6m));
            let from_6m_to_1y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From6mTo1y));
            let from_1y_to_2y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From1yTo2y));
            let from_2y_to_3y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From2yTo3y));
            let from_3y_to_5y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From3yTo5y));
            let from_5y_to_7y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From5yTo7y));
            let from_7y_to_10y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From7yTo10y));

            let from_1y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From1y));
            let from_10y_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::From10y));

            let year_2009_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2009));
            let year_2010_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2010));
            let year_2011_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2011));
            let year_2012_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2012));
            let year_2013_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2013));
            let year_2014_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2014));
            let year_2015_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2015));
            let year_2016_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2016));
            let year_2017_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2017));
            let year_2018_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2018));
            let year_2019_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2019));
            let year_2020_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2020));
            let year_2021_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2021));
            let year_2022_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2022));
            let year_2023_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2023));
            let year_2024_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::Year2024));

            let sth_handle =
                scope.spawn(|| UTXODataset::import(parent_path, UTXOCohortId::ShortTermHolders));

            let lth = UTXODataset::import(parent_path, UTXOCohortId::LongTermHolders)?;

            let mut s = Self {
                min_initial_state: MinInitialState::default(),

                cohorts: SplitByUTXOCohort {
                    up_to_1d: up_to_1d_handle.join().unwrap()?,
                    up_to_1w: up_to_1w_handle.join().unwrap()?,
                    up_to_1m: up_to_1m_handle.join().unwrap()?,
                    up_to_2m: up_to_2m_handle.join().unwrap()?,
                    up_to_3m: up_to_3m_handle.join().unwrap()?,
                    up_to_4m: up_to_4m_handle.join().unwrap()?,
                    up_to_5m: up_to_5m_handle.join().unwrap()?,
                    up_to_6m: up_to_6m_handle.join().unwrap()?,
                    up_to_1y: up_to_1y_handle.join().unwrap()?,
                    up_to_2y: up_to_2y_handle.join().unwrap()?,
                    up_to_3y: up_to_3y_handle.join().unwrap()?,
                    up_to_5y: up_to_5y_handle.join().unwrap()?,
                    up_to_7y: up_to_7y_handle.join().unwrap()?,
                    up_to_10y: up_to_10y_handle.join().unwrap()?,

                    from_1d_to_1w: from_1d_to_1w_handle.join().unwrap()?,
                    from_1w_to_1m: from_1w_to_1m_handle.join().unwrap()?,
                    from_1m_to_3m: from_1m_to_3m_handle.join().unwrap()?,
                    from_3m_to_6m: from_3m_to_6m_handle.join().unwrap()?,
                    from_6m_to_1y: from_6m_to_1y_handle.join().unwrap()?,
                    from_1y_to_2y: from_1y_to_2y_handle.join().unwrap()?,
                    from_2y_to_3y: from_2y_to_3y_handle.join().unwrap()?,
                    from_3y_to_5y: from_3y_to_5y_handle.join().unwrap()?,
                    from_5y_to_7y: from_5y_to_7y_handle.join().unwrap()?,
                    from_7y_to_10y: from_7y_to_10y_handle.join().unwrap()?,

                    from_1y: from_1y_handle.join().unwrap()?,
                    from_10y: from_10y_handle.join().unwrap()?,

                    sth: sth_handle.join().unwrap()?,
                    lth,

                    year_2009: year_2009_handle.join().unwrap()?,
                    year_2010: year_2010_handle.join().unwrap()?,
                    year_2011: year_2011_handle.join().unwrap()?,
                    year_2012: year_2012_handle.join().unwrap()?,
                    year_2013: year_2013_handle.join().unwrap()?,
                    year_2014: year_2014_handle.join().unwrap()?,
                    year_2015: year_2015_handle.join().unwrap()?,
                    year_2016: year_2016_handle.join().unwrap()?,
                    year_2017: year_2017_handle.join().unwrap()?,
                    year_2018: year_2018_handle.join().unwrap()?,
                    year_2019: year_2019_handle.join().unwrap()?,
                    year_2020: year_2020_handle.join().unwrap()?,
                    year_2021: year_2021_handle.join().unwrap()?,
                    year_2022: year_2022_handle.join().unwrap()?,
                    year_2023: year_2023_handle.join().unwrap()?,
                    year_2024: year_2024_handle.join().unwrap()?,
                },
            };

            s.min_initial_state
                .consume(MinInitialState::compute_from_datasets(&s));

            Ok(s)
        })
    }

    pub fn insert_data(&mut self, processed_block_data: &ProcessedBlockData) {
        self.cohorts
            .as_mut_vec()
            .into_iter()
            .for_each(|cohort| cohort.insert_data(processed_block_data))
    }

    fn as_vec(&self) -> Vec<&UTXODataset> {
        self.cohorts.as_vec()
    }

    fn as_mut_vec(&mut self) -> Vec<&mut UTXODataset> {
        self.cohorts.as_mut_vec()
    }
}

impl AnyDatasets for UTXODatasets {
    fn get_min_initial_state(&self) -> &MinInitialState {
        &self.min_initial_state
    }

    fn to_any_dataset_vec(&self) -> Vec<&(dyn AnyDataset + Send + Sync)> {
        self.as_vec()
            .into_iter()
            .map(|dataset| dataset as &(dyn AnyDataset + Send + Sync))
            .collect_vec()
    }

    fn to_mut_any_dataset_vec(&mut self) -> Vec<&mut dyn AnyDataset> {
        self.as_mut_vec()
            .into_iter()
            .map(|dataset| dataset as &mut dyn AnyDataset)
            .collect_vec()
    }
}
