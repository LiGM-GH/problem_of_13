use std::num::NonZeroU8;

use libsum13::integer;
use tap::Pipe;

#[allow(unused)]
use libsum13::traits::{SumSequencer, SumSequencerMut, SumSequencerOnce};

#[derive(Debug, Clone)]
pub struct BenchResult<T> {
    duration: std::time::Duration,
    value: T,
}

fn bench_it<T>(fun: impl FnOnce() -> T) -> BenchResult<T> {
    let now = std::time::Instant::now();

    let result = fun();

    let elapsed = now.elapsed();

    BenchResult {
        duration: elapsed,
        value: result,
    }
}

fn print_result<T: std::fmt::Debug>(
    name: &str,
) -> impl FnOnce(&BenchResult<T>) {
    move |val: &BenchResult<T>| {
        println!(
            "{:<20} | {:^20?} | {:>11.4?} ms",
            name,
            val.value,
            val.duration.as_nanos() as f64 / 1_000_000.0
        )
    }
}

fn measure_fun(value: impl SumSequencerOnce, iterations: u32, label: &str) {
    value
        .get_ints(iterations)
        .pipe(|val| bench_it(|| val.last().unwrap_or(u64::max_value())))
        .pipe_ref(print_result(label))
}

/// The problem is the following:
/// Find all the numbers that have sum of its digits equal to 13
fn main() {
    let iterations = 1_000_000;
    let sum = NonZeroU8::new(13).expect("Couldn't create NonZeroU8 from 13");

    println!("Iterations number is {iterations}");

    measure_fun(integer::WithDigitSum13 {}, iterations, "integer_static");

    measure_fun(integer::WithDigitSum(sum), iterations, "integer_dynamic");

    measure_fun(
        integer::WithDigitSumAdvanced(sum),
        iterations,
        "integer_advanced",
    );

    measure_fun(integer::FutureLooking(sum), iterations, "future_looking");

    bench_it(|| {
        integer::FullyPar(sum)
            .get_ints(iterations)
            .last()
            .unwrap_or(0)
    })
    .pipe_ref(print_result("fully_par (full)"));

    measure_fun(integer::FullyPar(sum), iterations, "fully_par (iters)");

    bench_it(|| integer::FullyPar(sum).get_ints(iterations))
        .pipe(|BenchResult { duration, value }| BenchResult {
            duration,
            value: value.last().unwrap_or(0),
        })
        .pipe_ref(print_result("fully_par (preproc)"));

    measure_fun(integer::SlowSequential(sum), iterations, "slow_sequential");
}
