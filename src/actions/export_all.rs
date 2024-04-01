use std::thread;

use chrono::{Datelike, Local, NaiveDate};

use crate::{databases::Databases, datasets::AllDatasets, states::States, utils::time};

pub struct ExportedData<'a> {
    pub databases: &'a mut Databases,
    pub datasets: &'a AllDatasets,
    pub date: NaiveDate,
    pub height: usize,
    pub states: &'a States,
}

pub fn export_all(
    ExportedData {
        databases,
        datasets,
        date,
        height,
        states,
    }: ExportedData,
) -> color_eyre::Result<()> {
    println!("{:?} - Saving... (Don't close !!)", Local::now());

    time("Total save time", || -> color_eyre::Result<()> {
        // time("States saved in", || states.export())?;

        // time("Databases saved in", || databases.export())?;

        // time("Datasets saved in", || {
        //     datasets.export_if_needed(date, height, export_data)
        // })

        thread::scope(|s| {
            s.spawn(|| time("Databases saved", || databases.export()));
            s.spawn(|| time("States saved", || states.export()));
            s.spawn(|| {
                time("Datasets saved", || {
                    datasets.export_if_needed(date, height, date.month0() == 0)
                })
            });
        });

        Ok(())
    })?;

    println!();

    Ok(())
}
