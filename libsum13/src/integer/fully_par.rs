use std::num::NonZeroU8;

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    either_iterator::EitherIterator, impl_mut_for_refmut, new_expect, traits::{SumSequencer, SumSequencerMut}
};

use super::{bounded::IntsWithDigitSumInBounds, count_iter_end, WithDigitSum};

pub struct FullyPar(pub NonZeroU8);
new_expect!(FullyPar);
impl_mut_for_refmut!(FullyPar);

impl SumSequencer for FullyPar {
    /// How to count the highest number (or at least its number of digits) ahead of time?
    ///
    /// Let's say we have 10 iterations.
    /// It's pretty obvious that we had 49 so it's 5 + 5 iterations which leads to 200 being the high end.
    /// Ok, then on 20 iterations? 49 means we have 5 iterations left on first hundred, then 15 left. Let's go until it gets to zero:
    /// ```markdown
    /// | hundred number | hundred's first | hundred's last | iterations count | looks like formula is |
    /// | -------        | ---------       | ---------      | ----------       | -----                 |
    /// | 0              | 49              | 94             | 5                | 5 + hundred_number    |
    /// | 1              | 139             | 193            | 6                | 5 + hundred_number    |
    /// | 2              | 229             | 292            | 7                | 5 + hundred_number    |
    /// | 3              | 319             | 391            | 8                | 5 + hundred_number    |
    /// | 4              | 409             | 490            | 9                | 5 + hundred_number    |
    /// | 5              | 508             | 580            | 8                | 13 - hundred_number   |
    /// | 6              | 607             | 670            | 7                | 13 - hundred_number   |
    /// | 7              | 706             | 760            | 6                | 13 - hundred_number   |
    /// | 8              | 805             | 850            | 5                | 13 - hundred_number   |
    /// | 9              | 904             | 940            | 4                | 13 - hundred_number   |
    /// | 10             | 1039            | 1093           | 6                | 5 + hundred_number    |
    /// | 11             | 1129            | 1192           | 7                | 5 + hundred_number    |
    /// | 12             | 1219            | 1291           | 8                | 5 + hundred_number    |
    /// | 13             | 1309            | 1390           | 9                | 5 + hundred_number    |
    /// | 14             | 1408            | 1480           | 8                | 13 - hundred_number   |
    /// | 15             | 1507            | 1570           | 7                | 13 - hundred_number   |
    /// | 16             | 1606            | 1660           | 6                | 13 - hundred_number   |
    /// | 17             | 1705            | 1750           | 5                | 13 - hundred_number   |
    /// | 18             | 1804            | 1840           | 4                | 13 - hundred_number   |
    /// | 19             | 1903            | 1930           | 3                | 13 - hundred_number   |
    /// | 20             | 2029            | 2092           | 7                | 5 + hundred_number    |
    /// ```
    ///
    /// Thus, something like
    /// ```rust,ignore
    /// fn get_iter_number(hundred_number: u64, initial: u64) -> u64 {
    ///     let digit_sum = hundred_number.digit_sum();
    ///     let num_iterations = min(13 - digit_sum, digit_sum + (100 - initial) / 9)
    /// }
    /// ```
    /// Going until the number is 0 should look like that:
    /// ```rust,ignore
    /// // function parameter
    /// let mut iterations;
    /// // pre-defined - struct field or constant
    /// let sum: NonZeroU8;
    ///
    /// let mut i = 0;
    /// let initial = get_initial(sum);
    ///
    /// let last_hundred_number = loop {
    ///     if iterations == 0 {
    ///         break;
    ///     }
    ///
    ///     iterations.saturating_sub(get_iter_number(i, initial));
    ///     i += 1;
    /// }
    /// ```
    fn get_ints(&self, iterations: u32) -> impl Iterator<Item = u64> + use<> {
        let last_number = count_iter_end(self.0, iterations);

        let num_threads = rayon::current_num_threads() as u64;

        if iterations as u64 <= num_threads * 100 {
            // TODO: Test if this is faster or slower than FutureLooking
            return EitherIterator::Left(
                WithDigitSum(self.0).get_ints(iterations),
            );
        }

        let sum_clone = self.0;

        EitherIterator::Right(
            std::iter::once_with(move || {
                let parts = (0..num_threads as u64)
                    .map(|i| {
                        let start = i * (last_number / num_threads);
                        let end = (i + 1) * (last_number / num_threads);
                        let sum = sum_clone;

                        IntsWithDigitSumInBounds { start, end, sum }
                    })
                    .collect::<Vec<_>>();

                let mut result = Vec::new();

                parts
                    .into_par_iter()
                    .map(|val| val.get_ints().collect::<Vec<_>>())
                    .collect_into_vec(&mut result);

                result.into_iter()
            })
            .flatten()
            .flatten()
            .take(iterations as usize),
        )
    }
}
