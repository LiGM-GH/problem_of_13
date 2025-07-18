use crate::{traits::{SumSequencer, SumSequencerMut}, DigitSum};
use std::num::NonZeroU8;

use crate::{impl_mut_for_refmut, new_expect};

use super::get_initial;

pub struct WithDigitSum(pub NonZeroU8);
new_expect!(WithDigitSum);
impl_mut_for_refmut!(WithDigitSum);

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

