//! Here are all the functions that use sequence of integers to calculate all numbers that have digits sum of 13

use std::num::NonZeroU8;

use rayon::prelude::*;

use crate::{
    DigitSum, get_initial, impl_mut_for_refmut, new_expect,
    traits::{SumSequencer, SumSequencerMut},
};

pub struct WithDigitSum13;
pub struct WithDigitSum(pub NonZeroU8);
pub struct WithDigitSumAdvanced(pub NonZeroU8);
pub struct FutureLooking(pub NonZeroU8);
pub struct FullyPar(pub NonZeroU8);
#[cfg(feature = "unstable_deprecated")]
pub struct NaivePar(pub NonZeroU8);
pub struct SlowSequential(pub NonZeroU8);

new_expect!(WithDigitSum);
new_expect!(WithDigitSumAdvanced);
new_expect!(FutureLooking);
new_expect!(FullyPar);
#[cfg(feature = "unstable_deprecated")]
new_expect!(NaivePar);
new_expect!(SlowSequential);

impl_mut_for_refmut!(WithDigitSum13);
impl_mut_for_refmut!(WithDigitSum);
impl_mut_for_refmut!(WithDigitSumAdvanced);
impl_mut_for_refmut!(FutureLooking);
impl_mut_for_refmut!(FullyPar);
#[cfg(feature = "unstable_deprecated")]
impl_mut_for_refmut!(NaivePar);
impl_mut_for_refmut!(SlowSequential);

impl SumSequencer for WithDigitSum13 {
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        std::iter::once(49).chain((0..iterations - 1).scan(49u64, |acc, _| {
            let next = *acc + 9;

            *acc = if next.digits_sum() == 13 {
                next
            } else {
                let mut next = (next - 8).next_multiple_of(100);

                while next.digits_sum() > 13 {
                    next = (next + 1).next_multiple_of(100);
                }

                let digits_sum = next.digits_sum();

                // Say digits_sum is 10.
                // Then the number is itself no less than 1900.
                // To get digits sum of 13 we need to add 3.
                // How do we know it's 3?
                // Well, it's 13 - 10, isn't it?
                // But what if  we had, say, 100?
                // Then we needed to add 39.
                // Why? Because 13 - 1 is 12 and 12 is more than 9
                // How much more is it? It's less by 3.

                let addition = if (13 - digits_sum) / 10 > 0 {
                    9 + (13 - digits_sum - 9) * 10
                } else {
                    13 - digits_sum
                };

                next + addition
            };

            Some(*acc)
        }))
    }
}

impl SumSequencer for WithDigitSum {
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        let sum = self.0;
        let initial = get_initial(sum);
        let sum = sum.get() as u64;

        std::iter::once(initial).chain((0..iterations - 1).scan(
            initial,
            move |acc, _| {
                let next = *acc + 9;

                *acc = if next.digits_sum() == sum {
                    next
                } else {
                    let mut next = (*acc + 1).next_multiple_of(100);

                    while next.digits_sum() > sum {
                        next = (next + 1).next_multiple_of(100);
                    }

                    let mut remainder = sum - next.digits_sum();
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

                    next + addition
                };

                Some(*acc)
            },
        ))
    }
}

impl SumSequencer for WithDigitSumAdvanced {
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        let sum = self.0;
        let initial = get_initial(sum);
        let sum = sum.get() as u64;

        std::iter::once(initial).chain((0..iterations - 1).scan(
            initial,
            move |acc, _| {
                let next = *acc + 9;

                *acc = if next.digits_sum() == sum {
                    next
                } else {
                    let mut next = *acc / 100 + 1;

                    let mut assumed = next.digits_sum();

                    while assumed > sum || sum - assumed >= 100 {
                        assumed += 1;

                        {
                            let mut elem = next;
                            while elem % 10 == 9 {
                                assumed -= 9;
                                elem /= 10;
                            }
                        }

                        next += 1;
                    }

                    next *= 100;

                    let mut remainder = sum - assumed;
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

                    next + addition
                };

                Some(*acc)
            },
        ))
    }
}

#[cfg(feature = "unstable_deprecated")]
impl SumSequencer for NaivePar {
    /// This function is here as a reminder that not everything that looks like an optimization is one.
    /// It is actually much slower than integer_dynamic and integer_static.
    /// The reasons are simple: each thread creation is actually a syscall.
    /// And on the micro-level, as it is done here, those "optimizations" are actually doing more harm
    /// than anything useful. The syscalls are much more costly than simple iteration.
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        let initial = get_initial(self.0);
        let sum = self.0.get() as u64;

        std::iter::once(initial).chain((0..iterations - 1).scan(
            initial,
            move |acc, _| {
                let next = *acc + 9;

                *acc = if next.digits_sum() == sum {
                    next
                } else {
                    let next = (*acc + 1).next_multiple_of(100);

                    let next_value = (0..)
                        .par_bridge()
                        .map(|i| next + 100 * i)
                        .find_first(|value| value.digits_sum() <= sum)
                        .expect(
                            "In the infinite range there should be such number",
                        );

                    let remainder = sum - next_value.digits_sum();

                    let addition = if remainder / 10 > 0 {
                        remainder * 10 - 81
                    } else {
                        remainder
                    };

                    next_value + addition
                };

                Some(*acc)
            },
        ))
    }
}

impl SumSequencer for FutureLooking {
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        let initial = get_initial(self.0);
        let sum = self.0.get() as u64;

        std::iter::once(initial).chain((0..iterations - 1).scan(
            initial,
            move |acc, _| {
                let next = *acc + 9;

                *acc = if next.digits_sum() == sum {
                    next
                } else {
                    let mut next = *acc / 100 + 1;

                    let mut assumed = next.digits_sum();

                    while assumed > sum || sum - assumed >= 100 {
                        assumed += 1;

                        {
                            let mut elem = next;
                            while elem % 10 == 9 {
                                assumed -= 9;
                                elem /= 10;
                            }
                        }

                        next += 1;
                    }

                    next *= 100;

                    let mut remainder = sum - assumed;
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

                    next + addition
                };

                Some(*acc)
            },
        ))
    }
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

impl SumSequencer for FullyPar {
    /// How to count the highest number (or at least its number of digits) ahead of time?
    ///
    /// Let's say we have 10 iterations.
    /// It's pretty obvious that we had 49 so it's 5 + 5 iterations which leads to 200 being the high end.
    /// Ok, then on 20 iterations? 49 means we have 5 iterations left on first hundred, then 15 left. Let's go until it gets to zero:
    /// ```markdown
    /// | hundred number | hundred's first | hundred's last | iterations count | looks like formula is |
    /// | -------        | ---------       | ---------      | ----------       | -----                 |
    /// | 0              | 49              | 94             | 5                | 5 + hundred_number    |
    /// | 1              | 139             | 193            | 6                | 5 + hundred_number    |
    /// | 2              | 229             | 292            | 7                | 5 + hundred_number    |
    /// | 3              | 319             | 391            | 8                | 5 + hundred_number    |
    /// | 4              | 409             | 490            | 9                | 5 + hundred_number    |
    /// | 5              | 508             | 580            | 8                | 13 - hundred_number   |
    /// | 6              | 607             | 670            | 7                | 13 - hundred_number   |
    /// | 7              | 706             | 760            | 6                | 13 - hundred_number   |
    /// | 8              | 805             | 850            | 5                | 13 - hundred_number   |
    /// | 9              | 904             | 940            | 4                | 13 - hundred_number   |
    /// | 10             | 1039            | 1093           | 6                | 5 + hundred_number    |
    /// | 11             | 1129            | 1192           | 7                | 5 + hundred_number    |
    /// | 12             | 1219            | 1291           | 8                | 5 + hundred_number    |
    /// | 13             | 1309            | 1390           | 9                | 5 + hundred_number    |
    /// | 14             | 1408            | 1480           | 8                | 13 - hundred_number   |
    /// | 15             | 1507            | 1570           | 7                | 13 - hundred_number   |
    /// | 16             | 1606            | 1660           | 6                | 13 - hundred_number   |
    /// | 17             | 1705            | 1750           | 5                | 13 - hundred_number   |
    /// | 18             | 1804            | 1840           | 4                | 13 - hundred_number   |
    /// | 19             | 1903            | 1930           | 3                | 13 - hundred_number   |
    /// | 20             | 2029            | 2092           | 7                | 5 + hundred_number    |
    /// ```
    ///
    /// Thus,
    /// ```rust,ignore
    /// fn get_iter_number(hundred_number: u64, initial: u64) -> u64 {
    ///     let digit_sum = hundred_number.digit_sum();
    ///     let num_iterations = min(13 - digit_sum, digit_sum + (100 - initial) / 9)
    /// }
    /// ```
    /// Going until the number is 0 should look like that:
    /// ```rust,ignore
    /// // function parameter
    /// let mut iterations;
    /// // pre-defined - struct field or constant
    /// let sum: NonZeroU8;
    ///
    /// let mut i = 0;
    /// let initial = get_initial(sum);
    ///
    /// let last_hundred_number = loop {
    ///     if iterations == 0 {
    ///         break;
    ///     }
    ///
    ///     iterations.saturating_sub(get_iter_number(i, initial));
    ///     i += 1;
    /// }
    /// ```
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        let last_number = count_iter_end(self.0, iterations);

        let num_threads = rayon::current_num_threads() as u64;

        if last_number <= num_threads * 100 {
            // TODO: Test if this is faster or slower than FutureLooking
            return EitherIterator::Left(
                WithDigitSum(self.0).get_ints(iterations),
            );
        }

        let sum_clone = self.0;

        EitherIterator::Right(
            std::iter::once_with(move || {
                let parts = (0..num_threads as u64)
                    .map(|i| {
                        let start = i * (last_number / num_threads);
                        let end = (i + 1) * (last_number / num_threads);
                        let sum = sum_clone;

                        IntsWithDigitSumInBounds { start, end, sum }
                    })
                    .collect::<Vec<_>>();

                let mut result = Vec::new();

                parts
                    .into_par_iter()
                    .map(|val| val.get_ints().collect::<Vec<_>>())
                    .collect_into_vec(&mut result);

                result.into_iter()
            })
            .flatten()
            .flatten()
            .take(iterations as usize),
        )
    }
}

enum EitherIterator<A, T1: Iterator<Item = A>, T2: Iterator<Item = A>> {
    Left(T1),
    Right(T2),
}

impl<A, T1: Iterator<Item = A>, T2: Iterator<Item = A>> Iterator
    for EitherIterator<A, T1, T2>
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Left(iter) => iter.next(),
            Self::Right(iter) => iter.next(),
        }
    }
}

#[derive(Debug)]
struct IntsWithDigitSumInBounds {
    start: u64,
    end: u64,
    sum: NonZeroU8,
}

pub fn count_iterations(sum: NonZeroU8, start: u64, end: u64) -> u64 {
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

pub fn count_iter_end(sum: NonZeroU8, iterations: u32) -> u64 {
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

impl IntsWithDigitSumInBounds {
    pub fn get_ints(&self) -> impl Iterator<Item = u64> + use<> {
        let start = (self.start / 100) * 100;
        let end = (self.end / 100) * 100;
        // let iterations = count_iterations(self.sum, self.start, self.end);
        let sum = self.sum.get() as u64;
        let sum_nonzerou8 = self.sum;

        let initial = {
            if start.digits_sum() == sum {
                start
            } else {
                let mut start_hundred = start / 100;

                let mut assumed = start_hundred.digits_sum();

                while (assumed > sum || sum - assumed >= 100)
                    && start_hundred * 100 <= end
                {
                    assumed += 1;

                    {
                        let mut elem = start_hundred;
                        while elem % 10 == 9 {
                            assumed -= 9;
                            elem /= 10;
                        }
                    }

                    start_hundred += 1;
                }

                let start_hundred = start_hundred * 100;

                let addition = count_addition(sum_nonzerou8, start_hundred);

                start_hundred + addition
            }
        };

        let inner_iter = (0..).scan(initial, move |acc, _| {
            let next = *acc + 9;

            *acc = if next.digits_sum() == sum {
                next
            } else {
                let mut next_hundred = *acc / 100 + 1;

                let mut assumed = next_hundred.digits_sum();

                while assumed > sum || sum - assumed >= 100 {
                    assumed += 1;

                    {
                        let mut elem = next_hundred;
                        while elem % 10 == 9 {
                            assumed -= 9;
                            elem /= 10;
                        }
                    }

                    next_hundred += 1;
                    if next_hundred * 100 > end {
                        return None;
                    }
                }

                let next_hundred = next_hundred * 100;

                let addition = count_addition(sum_nonzerou8, next_hundred);

                next_hundred + addition
            };

            if *acc >= end {
                return None;
            }

            Some(*acc)
        });

        if initial.digits_sum() == sum && initial < end {
            EitherIterator::Left(std::iter::once(initial).chain(inner_iter))
        } else {
            EitherIterator::Right(inner_iter)
        }
    }
}

impl SumSequencer for SlowSequential {
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        let sum_u64 = self.0.get() as u64;

        (0..)
            .scan(0, move |assumed, elem| {
                let will_return = *assumed == sum_u64;

                *assumed += 1;

                {
                    let mut elem = elem;
                    while elem % 10 == 9 {
                        *assumed -= 9;
                        elem /= 10;
                    }
                }

                if will_return {
                    return Some(Some(elem));
                }

                Some(None)
            })
            .flatten()
            .take(iterations as usize)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU8;

    use crate::{
        DigitSum, get_initial, integer::count_iterations, traits::SumSequencer,
    };

    use super::{IntsWithDigitSumInBounds, count_addition};

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
