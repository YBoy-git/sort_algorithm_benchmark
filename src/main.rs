use array_gen::{gen_random, gen_reversed, gen_sorted};
use bubble_sort::bubble_sort;
use core::fmt;
use modified_bubble_sort::modified_bubble_sort;
use plotters::prelude::*;
use shellsort::shellsort;
use std::{env, fs, path::Path};

const BENCH_DIR: &str = "bench_output";
const AVAILABLE_ALGORITHMS: [&str; 3] = ["bubble", "modified_bubble", "shell"];

#[derive(Debug)]
enum ArgError {
    NotEnoughArgs,
}
impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ArgError::*;
        match self {
            NotEnoughArgs => write!(f, "Not enough arguments"),
        }
    }
}
impl std::error::Error for ArgError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().collect::<Vec<_>>();

    if args.len() < 6 {
        return Err(Box::new(ArgError::NotEnoughArgs));
    }

    let mut start = args.remove(1).parse::<usize>()?;
    let end = args.remove(1).parse::<usize>()?;
    let step = args.remove(1).parse::<usize>()?;
    let time_top_bound = args.remove(1).parse::<f64>()?;
    let algorithm = args.remove(1);

    if start == 0 {
        start += step;
    }

    let algorithm_fn = match AVAILABLE_ALGORITHMS
        .iter()
        .position(|&alg| algorithm == alg)
    {
        None => panic!("Wrong algorithm"),
        Some(index) => match index {
            0 => bubble_sort,
            1 => modified_bubble_sort,
            2 => shellsort,
            _ => unreachable!("Not implemented algorithm"),
        },
    };

    let sizes = (start..=end).step_by(step).collect::<Vec<_>>();

    if !Path::new(BENCH_DIR).exists() {
        fs::create_dir(BENCH_DIR)?;
    }

    let file = format!("{}/{}_chart.png", BENCH_DIR, algorithm);

    let root_area = BitMapBackend::new(&file, (1080, 720)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption(format!("{} sort", algorithm), ("sans-serif", 40))
        .margin(100)
        .set_all_label_area_size(30)
        .build_cartesian_2d(start as f32..end as f32, 0f32..time_top_bound as f32)?;

    chart.configure_mesh().draw()?;

    let mut result_with_sorted = Vec::with_capacity(sizes.len());
    let mut result_with_reversed = Vec::with_capacity(sizes.len());
    let mut result_with_random = Vec::with_capacity(sizes.len());

    for size in sizes {
        let sorted_arr = gen_sorted(size);
        let reversed_arr = gen_reversed(size);
        let random_arr = gen_random(size);

        println!("Size: {}", size);
        println!();

        println!("\tSorted");
        {
            let arr = sorted_arr.clone();
            let metrics = algorithm_fn(arr);

            println!("\t\tComparisons: {}", metrics.comparisons);
            println!("\t\tSwaps: {}", metrics.swaps);
            println!("\t\tTime: {}ms", metrics.time.as_millis());

            result_with_sorted.push((size as f32, metrics.time.as_millis() as f32));
        }
        println!();

        println!("\tReversed");
        {
            let arr = reversed_arr.clone();
            let metrics = algorithm_fn(arr);

            println!("\t\tComparisons: {}", metrics.comparisons);
            println!("\t\tSwaps: {}", metrics.swaps);
            println!("\t\tTime: {}ms", metrics.time.as_millis());

            result_with_reversed.push((size as f32, metrics.time.as_millis() as f32));
        }
        println!();

        println!("\tAverage (random) case");
        {
            let arr = random_arr.clone();
            let metrics = algorithm_fn(arr);

            println!("\t\tComparisons: {}", metrics.comparisons);
            println!("\t\tSwaps: {}", metrics.swaps);
            println!("\t\tTime: {}ms", metrics.time.as_millis());

            result_with_random.push((size as f32, metrics.time.as_millis() as f32));
        }
        println!();
    }

    chart.draw_series(LineSeries::new(result_with_sorted, &GREEN))?;
    chart.draw_series(LineSeries::new(result_with_reversed, &RED))?;
    chart.draw_series(LineSeries::new(result_with_random, &BLUE))?;

    root_area.present()?;
    Ok(())
}
