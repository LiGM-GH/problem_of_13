use std::num::NonZeroU8;

use integer::count_iter_end;
use tap::Pipe;

#[allow(unused)]
use traits::{SumSequencer, SumSequencerMut, SumSequencerOnce};

mod combinatorics;
mod integer;
mod macros;
mod string;
mod traits;

trait DigitSum {
    fn digits_sum(&self) -> u64;
}

struct DigitIter(u64, u64);

impl Iterator for DigitIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let ret = self.0 % self.1;
            self.0 /= self.1;
            Some(ret)
        }
    }
}

impl DigitSum for u64 {
    fn digits_sum(&self) -> u64 {
        DigitIter(*self, 10).sum()
    }
}

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
            name, val.value, val.duration.as_nanos() as f64 / 1_000_000.0
        )
    }
}

fn measure_fun(value: impl SumSequencerOnce, iterations: u32, label: &str) {
    value
        .get_ints(iterations)
        .pipe(|val| bench_it(|| val.last()))
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

    measure_fun(integer::FutureLooking(sum), iterations, "future_looking");

    measure_fun(integer::FullyPar(sum), iterations, "fully_par");

    bench_it(|| integer::FullyPar(sum).get_ints(iterations).last())
        .pipe_ref(print_result("fully_par full"));

    bench_it(|| integer::FullyPar(sum).get_ints(iterations))
        .pipe(|BenchResult { duration, value }| BenchResult {
            duration,
            value: value.last(),
        })
        .pipe_ref(print_result("fully_par preproc"));

    bench_it(|| count_iter_end(sum, iterations))
        .pipe(|BenchResult { duration, value }| BenchResult {
            duration,
            value: Some(value),
        })
        .pipe_ref(print_result("fully_par count_end"));

    measure_fun(integer::SlowSequential(sum), iterations, "slow_sequential");
}

fn get_initial(sum: NonZeroU8) -> u64 {
    let mut sum_clone = sum.get();
    let mut first = 0u64;
    let mut i = 1;

    while sum_clone != 0 {
        match sum_clone.checked_sub(9) {
            Some(value) => {
                sum_clone = value;
                first += 9 * i;
            }
            None => {
                first += sum_clone as u64 * i;
                sum_clone = 0;
            }
        }

        i *= 10;
    }

    first
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, num::NonZeroU8};

    use crate::{
        bench_it, get_initial, integer, string, traits::SumSequencerOnce,
    };

    #[test]
    fn test_initial() {
        assert_eq!(get_initial(NonZeroU8::new(1).unwrap()), 1);
        assert_eq!(get_initial(NonZeroU8::new(2).unwrap()), 2);
        assert_eq!(get_initial(NonZeroU8::new(3).unwrap()), 3);
        assert_eq!(get_initial(NonZeroU8::new(4).unwrap()), 4);
        assert_eq!(get_initial(NonZeroU8::new(5).unwrap()), 5);
        assert_eq!(get_initial(NonZeroU8::new(6).unwrap()), 6);
        assert_eq!(get_initial(NonZeroU8::new(7).unwrap()), 7);
        assert_eq!(get_initial(NonZeroU8::new(8).unwrap()), 8);
        assert_eq!(get_initial(NonZeroU8::new(9).unwrap()), 9);
        assert_eq!(get_initial(NonZeroU8::new(10).unwrap()), 19);
        assert_eq!(get_initial(NonZeroU8::new(11).unwrap()), 29);
        assert_eq!(get_initial(NonZeroU8::new(12).unwrap()), 39);
        assert_eq!(get_initial(NonZeroU8::new(13).unwrap()), 49);
        assert_eq!(get_initial(NonZeroU8::new(14).unwrap()), 59);
        assert_eq!(get_initial(NonZeroU8::new(15).unwrap()), 69);
        assert_eq!(get_initial(NonZeroU8::new(16).unwrap()), 79);
        assert_eq!(get_initial(NonZeroU8::new(17).unwrap()), 89);
        assert_eq!(get_initial(NonZeroU8::new(18).unwrap()), 99);
        assert_eq!(get_initial(NonZeroU8::new(19).unwrap()), 199);
        assert_eq!(get_initial(NonZeroU8::new(20).unwrap()), 299);
    }

    #[test]
    fn test_int_variant() {
        let iterations = 10000;
        let strval = string::WithDigitSum13 {}.get_ints(iterations);
        let intval = integer::WithDigitSum13 {}.get_ints(iterations);

        strval
            .zip(intval)
            .for_each(|(left, right)| assert_eq!(left, right));
    }

    #[test]
    fn test_int_general() {
        let spawn_thing = |number: NonZeroU8| {
            let iterations = 2 * number.get() as u32;

            println!("iterations: {iterations}");

            string::WithDigitSum(number)
                .get_ints(iterations)
                .zip(integer::WithDigitSum(number).get_ints(iterations))
                .enumerate()
                .for_each(|(i, (left, right))| {
                    assert_eq!(
                        left, right,
                        "{i} didn't match: (string) {left} != (int) {right}"
                    )
                });
        };

        for i in 1..9 {
            spawn_thing(NonZeroU8::new(i).unwrap());
        }
    }

    #[test]
    fn test_int_general_again() {
        let iterations = 100_000;

        let intval = integer::WithDigitSum::new(13).get_ints(iterations);
        println!(
            "The last number of super integers is {:?}",
            bench_it(|| { intval.last() })
        );
    }

    #[test]
    fn test_fully_par_with_zip() {
        let iterations = 100_000;
        let intval = integer::WithDigitSum::new(13).get_ints(iterations);

        let super_val = integer::FullyPar::new(13).get_ints(iterations);

        let mut iter = intval.zip(super_val).peekable();
        let mut need_panic = false;

        while let Some((left, right)) = iter.next() {
            if left == right
                && let Some((left_peek, right_peek)) = iter.peek()
                && left_peek != right_peek
            {
                println!("{left} | {right} | {left_peek} | {right_peek}");
                need_panic = true;
            }
        }

        if need_panic {
            panic!();
        }
    }

    #[test]
    fn test_fully_par_with_hashsets() {
        let iterations = 100_000;
        let intval = integer::WithDigitSum::new(13)
            .get_ints(iterations)
            .take_while(|val| *val < iterations as u64);

        let super_val = integer::FullyPar::new(13)
            .get_ints(iterations)
            .take_while(|val| *val < iterations as u64);

        let int_result = intval.collect::<HashSet<_>>();
        let super_result = super_val.collect::<HashSet<_>>();

        assert_eq!(
            super_result
                .difference(&int_result)
                .copied()
                .collect::<Vec<u64>>(),
            vec![],
        );

        let mut diff = int_result
            .difference(&super_result)
            .copied()
            .collect::<Vec<u64>>();

        diff.sort();

        assert_eq!(diff, vec![]);
    }

    #[allow(unused, dead_code, deprecated)]
    #[cfg(feature = "unstable_deprecated")]
    mod deprecated {
        use std::collections::HashSet;

        use rayon::iter::{ParallelBridge, ParallelExtend};

        use crate::{bench_it, integer, traits::SumSequencer};

        #[test]
        fn test_int_super() {
            let iterations = 10000;

            let cool = integer::Rayon::new(13).get_ints(iterations);

            let intval = integer::WithDigitSum13 {}.get_ints(iterations);

            let mut cool_set = HashSet::new();
            cool_set.par_extend(cool.par_bridge());

            let mut int_set = HashSet::new();
            int_set.par_extend(intval.par_bridge());

            assert_eq!(int_set, cool_set);
        }

        #[test]
        fn test_super_int_again() {
            let iterations = 100_000;

            let intval = integer::Rayon::new(13).get_ints(iterations);
            let super_int = integer::WithDigitSum::new(13).get_ints(iterations);

            println!(
                "The last number of super integers is {:?}",
                bench_it(|| { intval.last() })
            );
            println!(
                "The last number of super integers is {:?}",
                bench_it(|| { super_int.last() })
            );
        }
    }
}
