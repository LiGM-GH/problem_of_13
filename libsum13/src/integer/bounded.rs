use std::num::NonZeroU8;

use crate::{either_iterator::EitherIterator, DigitSum};

use super::count_addition;

#[derive(Debug)]
pub struct IntsWithDigitSumInBounds {
    pub start: u64,
    pub end: u64,
    pub sum: NonZeroU8,
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

