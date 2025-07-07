pub trait SumSequencerMut {
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<Self>;
}

#[allow(refining_impl_trait)]
impl<T: SumSequencerMut> SumSequencerMut for &mut T {
    /// Since
    /// `impl IntsWithDigitSum for AnyPossibleT` doesn't reference AnyPossibleT
    /// then
    /// `impl IntsWithDigitSum for &mut AnyPossibleT` doesn't reference AnyPossibleT either.
    fn get_ints(
        &mut self,
        iterations: u32,
    ) -> impl Iterator<Item = u64> + use<T> {
        <T as SumSequencerMut>::get_ints(*self, iterations)
    }
}
