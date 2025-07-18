use crate::{
    DigitSum,
    impl_mut_for_refmut,
    traits::{SumSequencer, SumSequencerMut},
};

pub struct WithDigitSum13;
impl_mut_for_refmut!(WithDigitSum13);

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
