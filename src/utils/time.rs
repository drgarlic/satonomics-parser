use std::time::Instant;

use super::MILLISECONDS_IN_DAY;

pub fn time<F, T>(name: &str, function: F) -> T
where
    F: FnOnce() -> T,
{
    let time = Instant::now();

    let returned = function();

    println!("{name}: {} seconds", time.elapsed().as_secs_f32());

    returned
}

pub fn difference_in_days_between_timestamps(older: u32, younger: u32) -> u32 {
    (younger - older) / MILLISECONDS_IN_DAY as u32
}
