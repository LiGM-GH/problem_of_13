use std::num::NonZeroU8;

use crate::{
    impl_mut_for_refmut, new_expect,
    traits::{SumSequencer, SumSequencerMut}, DigitSum,
};

use super::get_initial;

pub struct FutureLooking(pub NonZeroU8);
new_expect!(FutureLooking);
impl_mut_for_refmut!(FutureLooking);

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
