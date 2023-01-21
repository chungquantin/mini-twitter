use std::time::Instant;

use colored::Colorize;

use super::log_stage;

pub fn start_benchmarking(stage: &'static str, title: &'static str) -> Instant {
    log_stage(stage, title);
    Instant::now()
}

pub fn stop_benchmarking(instant: Instant) {
    let elapsed = instant.elapsed();
    println!(
        "==> Total execution time: {}",
        format!("{:.2?}", elapsed).blue()
    );
}