use std::time::Instant;
use log::debug;

/// Times the execution of a function, logging duration only if `log` level is `debug`.
pub fn timer_debug<F, T>(label: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();

    debug!("⏱️ {} took {:?}", label, duration);
    result
}
