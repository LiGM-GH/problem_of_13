use std::num::NonZeroU8;

use tap::Pipe;
use traits::{SumSequencerMut, SumSequencerOnce};

mod combinatorics;
mod integer;
mod macros;
mod string;
mod traits;

trait DigitSum {
    fn digits_sum(&self) -> u64;
}

impl DigitSum for u64 {
    fn digits_sum(&self) -> u64 {
        let mut that = *self;
        let mut sum: u64 = 0;

        while that != 0 {
            sum += that % 10;
            that /= 10;
        }

        sum
    }
}

#[derive(Debug)]
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

fn print_result<T: std::fmt::Debug>(name: &str) -> impl FnOnce(BenchResult<T>) {
    move |val: BenchResult<T>| {
        println!(
            "The last number of the {:<20} is {:^20?} with duration of {:^20?}",
            name, val.value, val.duration
        )
    }
}

fn measure_fun(value: impl SumSequencerOnce, iterations: u32, label: &str) {
    value
        .get_ints(iterations)
        .pipe(|val| bench_it(|| val.last()))
        .pipe(print_result(label))
}

/// The problem is the following:
/// Find all the numbers that have sum of its digits equal to 13
fn main() {
    let iterations = 100_000;
    let sum = NonZeroU8::new(13).expect("Couldn't create NonZeroU8 from 13");

    measure_fun(integer::WithDigitSum13 {}, iterations, "integer_static");

    measure_fun(integer::WithDigitSum(sum), iterations, "integer_dynamic");

    measure_fun(integer::Rayon(sum), iterations, "rayon_dynamic");

    measure_fun(integer::FutureLooking(sum), iterations, "future_looking");

    let mut thing = integer::FutureLooking::new(20);
    measure_fun(&thing, 40, "40 iterations of future_looking of 20");
    thing = integer::FutureLooking::new(17);
    measure_fun(&thing, 40, "40 iterations of future_looking of 17");
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

    use rayon::iter::{ParallelBridge, ParallelExtend};

    use crate::{
        bench_it, get_initial, integer, string, traits::SumSequencerMut,
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
        let mut str_var = string::WithDigitSum13 {};
        let mut int_var = integer::WithDigitSum13 {};
        let strval = str_var.get_ints(iterations);
        let intval = int_var.get_ints(iterations);

        strval
            .zip(intval)
            .for_each(|(left, right)| assert_eq!(left, right));
    }

    #[test]
    fn test_int_super() {
        let iterations = 10000;

        let mut cool = integer::Rayon::new(13);
        let cool = cool.get_ints(iterations);

        let mut int_var = integer::WithDigitSum13 {};
        let intval = int_var.get_ints(iterations);

        let mut cool_set = HashSet::new();
        cool_set.par_extend(cool.par_bridge());

        let mut int_set = HashSet::new();
        int_set.par_extend(intval.par_bridge());

        assert_eq!(int_set, cool_set);
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

        let mut int_var = integer::WithDigitSum::new(13);
        let intval = int_var.get_ints(iterations);
        println!(
            "The last number of super integers is {:?}",
            bench_it(|| { intval.last() })
        );
    }

    #[test]
    fn test_super_int_again() {
        let iterations = 100_000;

        let mut int_var = integer::Rayon::new(13);
        let intval = int_var.get_ints(iterations);
        let mut super_int = integer::WithDigitSum::new(13);
        let super_int = super_int.get_ints(iterations);

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
