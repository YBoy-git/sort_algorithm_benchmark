use std::time::Duration;

#[derive(Default)]
pub struct Metrics {
    pub comparisons: usize,
    pub swaps: usize,
    pub time: Duration,
}