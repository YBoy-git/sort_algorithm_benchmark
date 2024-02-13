use std::time::Instant;

use metrics::Metrics;

pub fn bubble_sort<T: PartialOrd>(mut data: impl AsMut<[T]>) -> Metrics {
    let mut metrics = Metrics::default();
    let start = Instant::now();

    let data = data.as_mut();
    let n = data.len();

    let mut swapped = true;
    while swapped {
        metrics.comparisons += 1;

        swapped = false;
        for i in 1..n {
            metrics.comparisons += 1;

            if data[i - 1] > data[i] {
                data.swap(i - 1, i);
                metrics.swaps += 1;
                swapped = true;
            }
            metrics.comparisons += 1;
        }
        metrics.comparisons += 1;
    }
    metrics.comparisons += 1;

    metrics.time = start.elapsed();
    metrics
}