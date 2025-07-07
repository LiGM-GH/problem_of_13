#![macro_use]

#[macro_export]
macro_rules! new_expect {
    ($owner:ident) => {
        impl $owner {
            pub fn new(
                value: impl TryInto<NonZeroU8, Error: std::fmt::Debug>,
            ) -> Self {
                Self(value.try_into().expect("Digits sum must be nonzero"))
            }
        }
    };
}
