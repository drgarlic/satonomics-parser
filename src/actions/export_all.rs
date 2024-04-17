use std::thread;

use chrono::{Local, NaiveDate};

use crate::{databases::Databases, datasets::AllDatasets, states::States, utils::time};

pub struct ExportedData<'a> {
    pub databases: &'a mut Databases,
    pub datasets: &'a mut AllDatasets,
    pub date: NaiveDate,
    pub height: usize,
    pub states: &'a States,
}

pub fn export_all(
    ExportedData {
        databases,
        datasets,
        states,
        ..
    }: ExportedData,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving... (Don't close !!)", Local::now());

    time("Total save time", || -> color_eyre::Result<()> {
        time("Datasets saved", || datasets.export())?;

        thread::scope(|s| {
            s.spawn(|| time("Databases saved", || databases.export()));
            s.spawn(|| time("States saved", || states.export()));
        });

        Ok(())
    })?;

    println!();

    Ok(())
}
