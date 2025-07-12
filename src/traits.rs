pub trait SumSequencerOnce {
    fn get_ints(self, iterations: u32)
    -> impl Iterator<Item = u64> + use<Self>;
}

pub trait SumSequencerMut {
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<Self>;
}

pub trait SumSequencer {
    fn get_ints(
        &self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<Self>;
}

impl<T: SumSequencer> SumSequencerMut for T {
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<T> {
        SumSequencer::get_ints(self, iterations)
    }
}

impl<T: SumSequencerMut> SumSequencerOnce for T {
    fn get_ints(
        mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<T> {
        SumSequencerMut::get_ints(&mut self, iterations)
    }
}

#[allow(refining_impl_trait)]
impl<T: SumSequencer> SumSequencer for &T {
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<T> {
        SumSequencer::get_ints(*self, iterations)
    }
}

// // TODO: Figure out how to make this work
// #[allow(refining_impl_trait)]
// impl<T: SumSequencer> SumSequencerMut for &mut T {
//     /// Since
//     /// `impl IntsWithDigitSum for AnyPossibleT` doesn't reference AnyPossibleT
//     /// then
//     /// `impl IntsWithDigitSum for &mut AnyPossibleT` doesn't reference AnyPossibleT either.
//     fn get_ints(
//         &mut self,
//         iterations: u32,
//     ) -> impl Iterator<Item = u64> + use<T> {
//         <T as SumSequencerMut>::get_ints(*self, iterations)
//     }
// }
//
// #[allow(refining_impl_trait)]
// impl<T: SumSequencerMut> SumSequencerMut for &mut T {
//     /// Since
//     /// `impl IntsWithDigitSum for AnyPossibleT` doesn't reference AnyPossibleT
//     /// then
//     /// `impl IntsWithDigitSum for &mut AnyPossibleT` doesn't reference AnyPossibleT either.
//     fn get_ints(
//         &mut self,
//         iterations: u32,
//     ) -> impl Iterator<Item = u64> + use<T> {
//         <T as SumSequencerMut>::get_ints(*self, iterations)
//     }
// }

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::{SumSequencer, SumSequencerMut, SumSequencerOnce};

    #[test]
    #[allow(unused, dead_code, refining_impl_trait)]
    /// test by compilation - OK if compiles
    fn test_types() {
        #[allow(dead_code)]
        struct TestSumSequencer;

        impl SumSequencer for TestSumSequencer {
            fn get_ints(
                &self,
                iterations: u32,
            ) -> impl Iterator<Item = u64> + use<> {
                todo!();
                vec![].into_iter()
            }
        }

        #[allow(dead_code)]
        struct Test2SumSequencer;

        impl SumSequencerMut for Test2SumSequencer {
            fn get_ints(
                &mut self,
                iterations: u32,
            ) -> impl Iterator<Item = u64> + use<> {
                todo!();
                vec![].into_iter()
            }
        }

        impl SumSequencerMut for &mut Test2SumSequencer {
            fn get_ints(
                &mut self,
                iterations: u32,
            ) -> impl Iterator<Item = u64> + use<> {
                SumSequencerMut::get_ints(*self, iterations)
            }
        }

        #[allow(dead_code)]
        struct AssertSumSequencerMut<T: SumSequencerMut> {
            _inner: PhantomData<T>,
        }

        const _IS_SEQ_MUT: AssertSumSequencerMut<TestSumSequencer> =
            AssertSumSequencerMut {
                _inner: PhantomData {},
            };

        const _IS_SEQ_MUT_REF: AssertSumSequencerMut<&TestSumSequencer> =
            AssertSumSequencerMut {
                _inner: PhantomData {},
            };

        #[allow(dead_code)]
        struct AssertSumSequencerOnce<T: SumSequencerOnce> {
            _inner: PhantomData<T>,
        }

        const _IS_SEQ_ONCE: AssertSumSequencerOnce<TestSumSequencer> =
            AssertSumSequencerOnce {
                _inner: PhantomData {},
            };

        const _IS_SEQ_ONCE_REF: AssertSumSequencerOnce<&TestSumSequencer> =
            AssertSumSequencerOnce {
                _inner: PhantomData {},
            };

        const _MUT_IS_SEQ_MUT: AssertSumSequencerMut<Test2SumSequencer> =
            AssertSumSequencerMut {
                _inner: PhantomData {},
            };

        const _MUT_IS_SEQ_MUT_REF: AssertSumSequencerMut<
            &mut Test2SumSequencer,
        > = AssertSumSequencerMut {
            _inner: PhantomData {},
        };
    }
}
