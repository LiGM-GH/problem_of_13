//! Here are all the functions that use sequence of integers to calculate all numbers that have digits sum of 13

mod advanced;
mod bounded;
mod dynamic;
mod fully_par;
mod future_looking;
mod sequential;
mod statique;
#[cfg(feature = "unstable_deprecated")]
mod naive_par;

use std::num::NonZeroU8;

pub use advanced::WithDigitSumAdvanced;
pub use bounded::IntsWithDigitSumInBounds;
pub use dynamic::WithDigitSum;
pub use fully_par::FullyPar;
pub use future_looking::FutureLooking;
pub use sequential::SlowSequential;
pub use statique::WithDigitSum13;
#[cfg(feature = "unstable_deprecated")]
pub use naive_par::NaivePar;

use crate::DigitSum;

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

fn count_addition(sum: NonZeroU8, value: u64) -> u64 {
    let sum = sum.get() as u64;

    if sum < value.digits_sum() {
        return 0;
    }

    let mut remainder = sum - value.digits_sum();
    let mut addition = 0;
    let mut i = 1;

    while remainder != 0 {
        if remainder >= 9 {
            addition += 9 * i;
            remainder -= 9;
        } else {
            addition += remainder * i;
            remainder = 0;
        }
        i *= 10;
    }

    addition
}

fn count_iterations(sum: NonZeroU8, start: u64, end: u64) -> u64 {
    let initial = get_initial(sum);

    let start_hundred = start / 100;
    let end_hundred = end / 100;

    let mut assumed = start_hundred.digits_sum();

    let full_hundreds_iters = (start_hundred..end_hundred)
        .map(|i| {
            let result = 'inner: {
                let digit_sum = assumed;

                if digit_sum > sum.get() as u64 {
                    break 'inner 0;
                }

                let left = sum.get() as u64 - digit_sum;

                let mut result = left + 1;

                if initial <= 100
                    && let right = digit_sum + (100u64 - initial) / 9
                    && right < result
                {
                    result = right + 1;
                }

                result
            };

            assumed += 1;

            {
                let mut elem = i;
                while elem % 10 == 9 {
                    assumed -= 9;
                    elem /= 10;
                }
            }

            result
        })
        .sum::<u64>();

    let remainder = {
        let addition = count_addition(sum, end_hundred);
        // If addition is more than end % 100, it's zero.
        // Else it's the difference between them
        let addition = (end % 100).saturating_sub(addition);

        u64::min(
            addition / 9,
            (sum.get() as u64).saturating_sub(end_hundred.digits_sum()),
        )
    };

    full_hundreds_iters + remainder
}

pub(crate) fn count_iter_end(sum: NonZeroU8, iterations: u32) -> u64 {
    // TODO: This must be optimizable. It is now the slowest part of the FullyPar realization.
    let mut iterations = iterations as u64;
    let mut i = 0;
    let initial = get_initial(sum);
    let sum_u64 = sum.get() as u64;
    let mut assumed = 0;

    loop {
        if iterations == 0 {
            break i * 100;
        }

        iterations = iterations.saturating_sub('iter_count: {
            assumed += 1;

            {
                let mut elem = i;
                while elem % 10 == 9 {
                    assumed -= 9;
                    elem /= 10;
                }
            }

            let digit_sum = assumed;

            if digit_sum > sum_u64 {
                break 'iter_count 0;
            }

            let left = sum_u64 - digit_sum;

            let mut result = left + 1;

            if initial > 100 {
                break 'iter_count result;
            }

            let right = digit_sum + (100u64 - initial) / 9;

            if right < result {
                result = right + 1;
            }

            result
        });

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU8;

    use crate::{DigitSum, integer::count_iterations, traits::SumSequencer};

    use super::{IntsWithDigitSumInBounds, count_addition, get_initial};

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
        assert_eq!(get_initial(NonZeroU8::new(21).unwrap()), 399);
        assert_eq!(get_initial(NonZeroU8::new(22).unwrap()), 499);
        assert_eq!(get_initial(NonZeroU8::new(23).unwrap()), 599);
        assert_eq!(get_initial(NonZeroU8::new(24).unwrap()), 699);
        assert_eq!(get_initial(NonZeroU8::new(25).unwrap()), 799);
        assert_eq!(get_initial(NonZeroU8::new(35).unwrap()), 8999);
    }

    fn get_iter_number(
        sum: NonZeroU8,
        hundred_number: u64,
        initial: u64,
    ) -> u64 {
        // Ok, let's say we've got sum = 13, hundred_number = 5, initial = 49.
        // digit_sum = 5
        let digit_sum = hundred_number.digits_sum();

        // left = (13 - 5) = 8
        let Some(left) = (sum.get() as u64).checked_sub(digit_sum) else {
            return 0;
        };

        let mut result = left + 1;

        // right = 5 + (100 - 49) / 9 = 5 + (51 / 9) = 5 + 5 = 10
        if let Some(right) =
            (|| digit_sum.checked_add((100u64.checked_sub(initial)?) / 9))()
            && right < result
        {
            // right = 8. But is it right?
            // It's most clearly 508, 517, 526, 535, 544, 553, 562, 571, 580 which is 9.

            result = right + 1;
        }

        result
    }

    fn count_iterations_original(sum: NonZeroU8, start: u64, end: u64) -> u64 {
        let initial = get_initial(sum);

        let start_hundred = start / 100;
        let end_hundred = end / 100;

        let full_hundreds_iters = (start_hundred..end_hundred)
            .map(|i| get_iter_number(sum, i, initial))
            .sum::<u64>();

        let remainder = {
            let addition = count_addition(sum, end_hundred);
            let addition = (end % 100).saturating_sub(addition);

            u64::min(
                addition / 9,
                (sum.get() as u64).saturating_sub(end_hundred.digits_sum()),
            )
        };

        full_hundreds_iters + remainder
    }

    #[test]
    fn test_bounds() {
        println!("Start");

        let n = 40;
        println!(
            "{}th of ints is {:?}",
            n,
            crate::integer::WithDigitSum13 {}.get_ints(10000).nth(n)
        );

        let test_range =
            |from, to| {
                IntsWithDigitSumInBounds {
                    start: from,
                    end: to,
                    sum: NonZeroU8::new(13).unwrap(),
                }
                .get_ints()
                .zip(crate::integer::WithDigitSum13 {}.get_ints(10000).skip(
                    count_iterations(NonZeroU8::new(13).unwrap(), 0, from)
                        as usize,
                ))
                .for_each(|(l, r)| assert_eq!(l, r));
                println!("{to} OK");
            };

        test_range(0, 500);
        test_range(500, 1000);
        test_range(1000, 5000);
        test_range(5000, 10000);
        test_range(10000, 50000);
        test_range(50000, 100_000);
        test_range(100_000, 200_000);
        test_range(37500, 50000);
    }

    #[test]
    fn test_count_iters() {
        let sum = NonZeroU8::new(13).unwrap();
        assert_eq!(count_iterations(sum, 0, 100), 6);
        assert_eq!(count_iterations(sum, 0, 200), 13);
        assert_eq!(count_iterations(sum, 0, 200), 13);
    }

    #[test]
    fn test_count_iters_original() {
        let sum = NonZeroU8::new(13).unwrap();

        assert_eq!(count_iterations_original(sum, 0, 100), 6);
        assert_eq!(count_iterations_original(sum, 0, 200), 13);
        assert_eq!(count_iterations_original(sum, 0, 200), 13);
    }

    #[test]
    fn test_count_iters_cross() {
        let sum = NonZeroU8::new(13).unwrap();
        let mut will_panic = false;
        let mut test_range = |from, to| {
            let count_optim = count_iterations(sum, from, to);
            let count_orig = count_iterations_original(sum, from, to);

            if count_optim != count_orig {
                println!(
                    "range: {from} to {to}: optim {count_optim}, orig {count_orig}"
                );
                will_panic = true;
            }
        };

        test_range(0, 500);
        test_range(500, 1000);
        test_range(1000, 5000);
        test_range(5000, 10000);
        test_range(10000, 50000);
        test_range(50000, 100_000);
        test_range(100_000, 200_000);
        test_range(37500, 50000);

        if will_panic {
            panic!()
        }
    }
}
