use crate::{
    DigitSum, impl_mut_for_refmut,
    integer::get_initial,
    new_expect,
    traits::{SumSequencer, SumSequencerMut},
};

use rayon::iter::{ParallelBridge, ParallelIterator};

use std::num::NonZeroU8;

pub struct NaivePar(pub NonZeroU8);

new_expect!(NaivePar);

impl_mut_for_refmut!(NaivePar);

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
