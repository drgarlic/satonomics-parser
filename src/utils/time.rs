use std::time::Instant;

pub fn time<F, T>(name: &str, function: F) -> T
where
    F: FnOnce() -> T,
{
    let time = Instant::now();

    let returned = function();

    println!("{name}: {} seconds", time.elapsed().as_secs_f32());

    returned
}
