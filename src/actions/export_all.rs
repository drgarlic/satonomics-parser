use chrono::{Datelike, Local, NaiveDate};

use crate::{
    databases::Databases,
    datasets::{AllDatasets, AnyDatasets},
    states::States,
    utils::time,
};

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

    // thread::scope(|s| {
    //     s.spawn(|| databases.export());
    //     s.spawn(|| states.export());
    // });

    let _ = time("databases", || databases.export());

    let _ = time("states", || states.export());

    time("datasets", || {
        datasets.export_if_needed(date, height, date.day0() == 0 && date.month0() == 0)
    })?;

    Ok(())
}
