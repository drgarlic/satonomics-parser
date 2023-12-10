use bitcoin_explorer::BitcoinDB;

use crate::structs::HeightMap;

#[allow(dead_code)]
pub fn compute_height_to_price(
    _db: &BitcoinDB,
    _block_count: usize,
) -> color_eyre::Result<HeightMap<f32>> {
    println!("Computing height_to_price...");

    // let har_prices = read_binance_har_file()?;
    // let latest_prices = fetch_1mn_prices_from_kraken()?;

    let height_to_price = HeightMap::new("height_to_price.json", true);

    // let start = height_to_price.get_first_unsafe_height().unwrap_or(0);

    // println!("height_to_price");
    // println!("Start at index: {start}");

    // db.iter_block::<FBlock>(start, block_count)
    //     .enumerate()
    //     .try_for_each(|(index, block)| -> color_eyre::Result<()> {
    //         let height = start + index;

    //         if height_to_price.get(height).is_some() {
    //             return Ok(());
    //         }

    //         let timestamp = Utc.timestamp_opt(i64::from(block.header.time), 0).unwrap();
    //         let timestamp = NaiveDateTime::new(
    //             timestamp.date_naive(),
    //             NaiveTime::from_hms_opt(timestamp.hour(), timestamp.minute(), 0).unwrap(),
    //         )
    //         .timestamp() as u32;

    //         if let Some(price) = latest_prices.get(&timestamp) {
    //             println!("For {height} - {timestamp} found {price} (kraken)");
    //             height_to_price.insert(height, price.to_owned());
    //         } else if let Some(price) = har_prices.get(&timestamp) {
    //             println!("For {height} - {timestamp} found {price} (binance)");
    //             height_to_price.insert(height, price.to_owned());
    //         } else {
    //             panic!(
    //                 "Can't find price for {height} - {timestamp}, please update binance.har file"
    //             )
    //         }

    //         Ok(())
    //     })?;

    // height_to_price.export()?;

    Ok(height_to_price)
}
