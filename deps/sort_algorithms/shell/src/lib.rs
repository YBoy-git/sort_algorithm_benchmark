use std::time::Instant;

use metrics::Metrics;

pub fn shellsort<T: PartialOrd>(data: &mut [T]) -> Metrics {
    let mut metrics = Metrics::default();
    let start = Instant::now();

    let n = data.len();

    // Find the closest Knuth member
    let mut gap = 1;
    while gap < n / 3 {
        metrics.comparisons += 1;

        gap = 3 * gap + 1;
    }
    metrics.comparisons += 1;

    // Main logic
    while gap >= 1 {
        metrics.comparisons += 1;

        for i in gap..n {
            metrics.comparisons += 1;

            let mut j = i;
            while j >= gap && data[j - gap] > data[j] {
                metrics.comparisons += 3;

                data.swap(j - gap, j);
                metrics.swaps += 1;
                j -= gap;
            }

            metrics.comparisons += 3; // assuming the worst
        }
        metrics.comparisons += 1;

        // Algorithm step
        gap /= 3;
    }
    metrics.comparisons += 1;

    metrics.time = start.elapsed();
    metrics
}
