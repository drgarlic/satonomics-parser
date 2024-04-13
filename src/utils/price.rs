pub fn convert_price_to_significant_cents(price: f32) -> u64 {
    let mut price_in_cents = (price * 100.0).round() as u64;

    let ilog10 = price_in_cents.checked_ilog10().unwrap_or(0) as i32;

    let significant_digits = 4;

    if ilog10 >= significant_digits {
        let log_diff = ilog10 - significant_digits + 1;

        let pow = 10.0_f64.powi(log_diff);

        price_in_cents = ((price_in_cents as f64 / pow).round() * pow) as u64;
    }

    price_in_cents
}
