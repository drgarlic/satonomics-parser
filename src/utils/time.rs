use std::time::Instant;

pub fn time<F, T>(name: &str, func: F) -> T
where
    F: FnOnce() -> T,
{
    let time = Instant::now();

    let out = func();

    println!("{name}: {} seconds\n", time.elapsed().as_secs_f32());

    out
}
