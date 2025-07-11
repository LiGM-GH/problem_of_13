#![macro_use]

#[macro_export]
macro_rules! new_expect {
    ($owner:ident) => {
        #[allow(dead_code)]
        impl $owner {
            pub fn new(
                value: impl TryInto<NonZeroU8, Error: std::fmt::Debug>,
            ) -> Self {
                Self(value.try_into().expect("Digits sum must be nonzero"))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mut_for_refmut {
    ($owner:ident) => {
        #[allow(refining_impl_trait)]
        #[allow(dead_code)]
        impl SumSequencerMut for &mut $owner {
            fn get_ints(
                &mut self,
                iterations: u32,
            ) -> impl Iterator<Item = u64> + use<> {
                SumSequencerMut::get_ints(*self, iterations)
            }
        }
    };
}
