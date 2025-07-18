pub enum EitherIterator<A, T1: Iterator<Item = A>, T2: Iterator<Item = A>> {
    Left(T1),
    Right(T2),
}

impl<A, T1: Iterator<Item = A>, T2: Iterator<Item = A>> Iterator
    for EitherIterator<A, T1, T2>
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Left(iter) => iter.next(),
            Self::Right(iter) => iter.next(),
        }
    }
}

