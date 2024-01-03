use crate::{
    structs::DateMap,
    utils::{string_to_naive_date, Kraken},
};

pub fn compute_date_to_price() -> color_eyre::Result<DateMap<f32>> {
    println!("Computing date_to_price...");

    let date_to_price = DateMap::import("date_to_price.json");

    Kraken::fetch_daily_prices()?
        .iter()
        .for_each(|(date, price)| {
            println!("Date: {date}");

            date_to_price.insert(&string_to_naive_date(date), price.to_owned());
        });

    date_to_price.export()?;

    Ok(date_to_price)
}
