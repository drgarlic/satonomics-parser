pub fn convert_sats_to_bitcoins(sats: u64) -> f64 {
    sats as f64 / 100_000_000.0
}
