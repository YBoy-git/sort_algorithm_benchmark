use std::time::Instant;

use metrics::Metrics;

pub fn modified_bubble_sort<T: PartialOrd>(data: &mut [T]) -> Metrics {
    let mut metrics = Metrics::default();
    let start = Instant::now();

    let mut n = data.len();

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

        n -= 1;
    }
    metrics.comparisons += 1;

    metrics.time = start.elapsed();
    metrics
}
