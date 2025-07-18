//! Here are all the functions that use sequence of strings  to calculate all numbers that have digits sum of 13

use std::num::NonZeroU8;

use crate::{
    new_expect,
    traits::{SumSequencer, SumSequencerMut},
};

pub struct WithDigitSum13;
pub struct WithDigitSum(pub NonZeroU8);

new_expect!(WithDigitSum);
impl_mut_for_refmut!(WithDigitSum);
impl_mut_for_refmut!(WithDigitSum13);

impl SumSequencer for WithDigitSum13 {
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        (0..iterations).scan(49, |acc, _| {
            while acc
                .to_string()
                .chars()
                .map(|digit| {
                    digit.to_string().parse::<u64>().expect(
                    "Couldn't parse digit from previously stringified digit",
                )
                })
                .sum::<u64>()
                != 13
            {
                *acc += 1;
            }

            let result = *acc;

            *acc += 1;

            Some(result)
        })
    }
}

impl SumSequencer for WithDigitSum {
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        let sum = self.0;
        let sum = sum.get();

        (0..iterations).scan(0, move |acc, _| {
            while acc
                .to_string()
                .chars()
                .map(|digit| {
                    digit.to_string().parse::<u8>().expect(
                    "Couldn't parse digit from previously stringified digit",
                )
                })
                .sum::<u8>()
                != sum
            {
                *acc += 1;
            }

            let result = *acc;

            *acc += 1;

            Some(result)
        })
    }
}
