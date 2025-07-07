//! Here are all the functions that use sequence of integers to calculate all numbers that have digits sum of 13

use std::num::NonZeroU8;

use rayon::prelude::*;

use crate::{DigitSum, get_initial, new_expect, traits::SumSequencerMut};

pub struct WithDigitSum13;
pub struct WithDigitSum(pub NonZeroU8);
pub struct FutureLooking(pub NonZeroU8);
pub struct FullyPar(pub NonZeroU8);
pub struct Rayon(pub NonZeroU8);

new_expect!(WithDigitSum);
new_expect!(FutureLooking);
new_expect!(FullyPar);
new_expect!(Rayon);

impl SumSequencerMut for WithDigitSum13 {
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<> {
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

impl SumSequencerMut for WithDigitSum {
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<> {
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

                    let remainder = sum - next.digits_sum();

                    let addition = if remainder / 10 > 0 {
                        remainder * 10 - 81
                    } else {
                        remainder
                    };

                    next + addition
                };

                Some(*acc)
            },
        ))
    }
}

impl SumSequencerMut for Rayon {
    /// This function is here as a reminder that not everything that looks like an optimization is one.
    /// It is actually much slower than integer_dynamic and integer_static.
    /// The reasons are simple: each thread creation is actually a syscall.
    /// And on the micro-level, as it is done here, those "optimizations" are actually doing more harm
    /// than anything useful. The syscalls are much more costly than simple iteration.
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<> {
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

impl SumSequencerMut for FutureLooking {
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<> {
        let initial = get_initial(self.0);
        let sum = self.0.get() as u64;
        let sum_nonzerou8 = self.0;

        std::iter::once(initial).chain((0..iterations - 1).scan(
            initial,
            move |acc, _| {
                let next = *acc + 9;

                *acc = if next.digits_sum() == sum {
                    next
                } else {
                    let next = (*acc + 1).next_multiple_of(100);

                    let next_value = (0..)
                        .map(|i| next + 100 * i)
                        .find(|value| value.digits_sum() <= sum)
                        .expect(
                            "In the infinite range there should be such number",
                        );

                    let addition = count_addition(sum_nonzerou8, next_value);

                    next_value + addition
                };

                Some(*acc)
            },
        ))
    }
}

fn count_addition(sum: NonZeroU8, value: u64) -> u64 {
    let remainder = sum.get() as u64 - value.digits_sum();

    if remainder / 10 > 0 {
        // NOTE: actually, (10 * (remainder - 9)) + 9
        remainder * 10 - 81
    } else {
        remainder
    }
}

impl SumSequencerMut for FullyPar {
    /// NOTE: How to count the highest number (or at least its number of digits) ahead of time?
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
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<> {
        let num_threads = rayon::current_num_threads() as u32;

        if iterations <= num_threads * 100 {
            // TODO: test if this is faster or slower than FutureLooking
            return EitherIterator::Left(
                WithDigitSum(self.0).get_ints(iterations),
            );
        }

        // TODO: Divide the interval into num_threads intervals,
        // then create IntsWithDigitSumInBounds for them and then zip them
        todo!();
        EitherIterator::Right(vec![].into_iter())
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

struct IntsWithDigitSumInBounds {
    start: u64,
    end: u64,
    sum: NonZeroU8,
}

fn count_iterations(sum: NonZeroU8, start: u64, end: u64) -> u64 {
    todo!()
}

impl IntsWithDigitSumInBounds {
    pub fn get_ints(&self) -> impl Iterator<Item = u64> {
        let initial = count_addition(self.sum, self.start);
        let iterations = count_iterations(self.sum, self.start, self.end);
        let sum = self.sum.get() as u64;
        let sum_nonzerou8 = self.sum;
        std::iter::once(initial).chain((0..iterations - 1).scan(
            initial,
            move |acc, _| {
                let next = *acc + 9;

                *acc = if next.digits_sum() == sum {
                    next
                } else {
                    let next = (*acc + 1).next_multiple_of(100);

                    let next_value = (0..)
                        .map(|i| next + 100 * i)
                        .find(|value| value.digits_sum() <= sum)
                        .expect(
                            "In the infinite range there should be such number",
                        );

                    let addition = count_addition(sum_nonzerou8, next_value);

                    next_value + addition
                };

                Some(*acc)
            },
        ))
    }
}
