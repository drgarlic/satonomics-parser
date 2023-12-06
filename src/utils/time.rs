use std::time::Instant;

pub fn time<F, T>(func: F) -> color_eyre::Result<T>
where
    F: FnOnce() -> color_eyre::Result<T>,
{
    let time = Instant::now();

    let res = func()?;

    println!("Took {} seconds\n", time.elapsed().as_secs_f32());

    Ok(res)
}
