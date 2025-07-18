use std::num::NonZeroU8;

use crate::{
    impl_mut_for_refmut, new_expect,
    traits::{SumSequencer, SumSequencerMut},
};

pub struct SlowSequential(pub NonZeroU8);
new_expect!(SlowSequential);
impl_mut_for_refmut!(SlowSequential);

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
