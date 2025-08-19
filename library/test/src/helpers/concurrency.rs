//! Helper module which helps to determine amount of threads to be used
//! during tests execution.
#[cfg(not(target_arch = "bpf"))]
use std::{env, num::NonZero, thread};

#[cfg(not(target_arch = "bpf"))]
pub fn get_concurrency() -> usize {
    if let Ok(value) = env::var("RUST_TEST_THREADS") {
        match value.parse::<NonZero<usize>>().ok() {
            Some(n) => n.get(),
            _ => panic!("RUST_TEST_THREADS is `{value}`, should be a positive integer."),
        }
    } else {
        thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
    }
}

#[cfg(target_arch = "bpf")]
pub fn get_concurrency() -> usize {
    1
}
