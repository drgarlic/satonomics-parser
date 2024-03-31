use chrono::NaiveDate;
use rayon::prelude::*;

use crate::datasets::{ExportData, ProcessedBlockData, ProcessedDateData};

use super::{GenericDataset, MinInitialState};

pub trait AnyDatasets {
    fn get_min_initial_state(&self) -> &MinInitialState;

    fn to_generic_dataset_vec(&self) -> Vec<&(dyn GenericDataset + Send + Sync)>;

    fn insert_date_data(&self, processed_date_data: ProcessedDateData) {
        let ProcessedDateData { date, .. } = processed_date_data;

        self.to_generic_dataset_vec()
            .par_iter()
            .filter(|dataset| dataset.should_insert_date(date))
            .for_each(|dataset| dataset.insert_date_data(&processed_date_data));
    }

    fn insert_block_data(&self, processed_block_data: ProcessedBlockData) {
        let ProcessedBlockData { height, date, .. } = processed_block_data;

        self.to_generic_dataset_vec()
            .par_iter()
            .filter(|dataset| dataset.should_insert(height, date))
            .for_each(|dataset| dataset.insert_block_data(&processed_block_data));
    }

    fn export_if_needed(
        &self,
        date: NaiveDate,
        height: usize,
        compute: bool,
    ) -> color_eyre::Result<()> {
        self.to_generic_dataset_vec()
            .iter()
            .filter(|dataset| dataset.should_insert(height, date))
            .try_for_each(|dataset| -> color_eyre::Result<()> {
                dataset.prepare();

                // if compute {
                //     dataset.compute(ExportData {})
                // }

                dataset.export()
            })?;

        Ok(())
    }

    fn export(&self) -> color_eyre::Result<()> {
        self.to_generic_dataset_vec()
            .iter()
            .try_for_each(|dataset| {
                dataset.prepare();

                // dataset.compute(ExportData {})

                dataset.export()
            })?;

        Ok(())
    }
}
